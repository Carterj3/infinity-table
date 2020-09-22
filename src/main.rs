use ::rppal::spi::{Bus, Mode, SlaveSelect, Spi};

use ::clap::{App, Arg};

use ::tokio::{process::Command, sync::RwLock};
use ::warp::Filter;

use ::anyhow::{anyhow, Context};

use ::infinity_table::{
    modes::{chaser::Chaser, constant::Constant, off::Off, wave::Wave, Mode as TableMode},
    Led,
};

use ::log::{error, trace};

use ::std::convert::Infallible;
use ::std::sync::Arc;

struct Config {
    num_end_frames: usize,
    led_offset: usize,
    num_leds: usize,
    spi_hz: u32,
    brightness: u8,
    cycle_delay_ms: u64,
}

impl Config {
    fn num_lit_leds(&self) -> usize {
        self.num_leds - self.led_offset
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let matches = App::new("Infinity Table")
        .version("1.0")
        .about("Controls an APA102 LED strip within an Infinity Table")
        .arg(
            Arg::with_name("num_end_frames")
                .help("Number of end frames to use")
                .required(true),
        )
        .arg(
            Arg::with_name("led_offset")
                .help("Number of LEDs to skip before the first lit LED")
                .required(true),
        )
        .arg(
            Arg::with_name("num_leds")
                .help("Number of total LEDs in the table")
                .required(true),
        )
        .arg(
            Arg::with_name("spi_hz")
                .help("Number of hertz for spi")
                .required(true),
        )
        .arg(
            Arg::with_name("brightness")
                .help("Brightness value (0-31)")
                .required(true),
        )
        .arg(
            Arg::with_name("cycle_delay_ms")
                .help("MS per cycle")
                .required(true),
        )
        .arg(
            Arg::with_name("max_temp")
                .help("*C before the mode is set to OFF. Whole number (ex. `30`)")
                .required(true),
        )
        .after_help("Example: `infinity-table 19 0 300 1000000 11 25 60`")
        .get_matches();

    let config = Arc::new(Config {
        num_end_frames: clap::value_t!(matches.value_of("num_end_frames"), usize).unwrap(),
        led_offset: clap::value_t!(matches.value_of("led_offset"), usize)
            .expect("num_leds should have been set"),
        num_leds: clap::value_t!(matches.value_of("num_leds"), usize)
            .expect("num_leds should have been set"),
        spi_hz: clap::value_t!(matches.value_of("spi_hz"), u32)
            .expect("spi_hz should have been set"),
        brightness: 0xE0 | clap::value_t!(matches.value_of("brightness"), u8).unwrap(),
        cycle_delay_ms: clap::value_t!(matches.value_of("cycle_delay_ms"), u64).unwrap(),
    });
    let max_temp =
        clap::value_t!(matches.value_of("max_temp"), usize).expect("max_temp should have been set");

    let led_mode: Arc<RwLock<Box<dyn TableMode>>> = Arc::new(RwLock::new(Box::new(Wave::new(
        config.num_lit_leds(),
        config.brightness.clone(),
    ))));

    let mode_filter = {
        let led_mode = led_mode.clone();
        warp::any().map(move || led_mode.clone())
    };

    let config_filter = {
        let config = config.clone();
        warp::any().map(move || config.clone())
    };

    {
        let config = config.clone();
        let led_mode = led_mode.clone();
        tokio::spawn(async move {
            tokio::time::delay_for(std::time::Duration::from_millis(1500)).await;

            let mut spi =
                Spi::new(Bus::Spi0, SlaveSelect::Ss0, config.spi_hz, Mode::Mode0).unwrap();

            loop {
                let mut leds: Vec<u8> = Vec::new();
                {
                    let mut mode = led_mode.write().await;
                    // Start frame
                    leds.append(&mut vec![0, 0, 0, 0]);
                    // Blank farmes
                    for _ in 0..config.led_offset {
                        leds.append(&mut vec![0xE0, 0u8, 0u8, 0u8]);
                    }
                    // Driver frames
                    for led in mode.advance().iter() {
                        let bytes = led.as_bytes();
                        leds.append(&mut vec![bytes[0], bytes[1], bytes[2], bytes[3]]);
                    }
                    // End frames
                    for _ in 0..config.num_end_frames {
                        leds.append(&mut vec![0x00, 0x00, 0x00, 0x00]);
                    }
                }

                tokio::time::delay_for(std::time::Duration::from_millis(config.cycle_delay_ms))
                    .await;
                // TODO: Maybe don't crash the program? Or at least also restart the raspi?
                if let Err(e) = spi.write(&leds) {
                    error!("Failed to write to LEDs: {:?}", e);
                } else {
                    trace!("Broadcasted LEDs");
                }
            }
        });
    }

    {
        let config = config.clone();
        let led_mode = led_mode.clone();
        tokio::spawn(async move {
            // ex. 47774
            match get_cpu_temp().await {
                Ok(cpu_temp) => {
                    if cpu_temp > (max_temp * 1000) {
                        {
                            let mut mode = led_mode.write().await;
                            *mode = Box::new(Off::new(config.num_lit_leds()));
                        }
                    }
                }
                otherwise => {
                    log::error!("Failed to read cpu temp: {:?}", otherwise);
                }
            }

            // ex. 47.8 | but the decimal gets truncated earlier
            match get_gpu_temp().await {
                Ok(gpu_temp) => {
                    if gpu_temp > max_temp {
                        {
                            let mut mode = led_mode.write().await;
                            *mode = Box::new(Off::new(config.num_lit_leds()));
                        }
                    }
                }
                otherwise => {
                    log::error!("Failed to read gpu temp: {:?}", otherwise);
                }
            }

            trace!("Temps are valid");
            tokio::time::delay_for(std::time::Duration::from_secs(15)).await;
        });
    }

    let wave = warp::get()
        .and(warp::path("wave"))
        .and(mode_filter.clone())
        .and(config_filter.clone())
        .and_then(wave_http);
    let chaser = warp::get()
        .and(warp::path("chaser"))
        .and(mode_filter.clone())
        .and(config_filter.clone())
        .and_then(chaser_http);

    let red = warp::get()
        .and(warp::path("red"))
        .and(mode_filter.clone())
        .and(config_filter.clone())
        .and_then(red_http);
    let green = warp::get()
        .and(warp::path("green"))
        .and(mode_filter.clone())
        .and(config_filter.clone())
        .and_then(green_http);
    let blue = warp::get()
        .and(warp::path("blue"))
        .and(mode_filter.clone())
        .and(config_filter.clone())
        .and_then(blue_http);

    let off = warp::get()
        .and(warp::path("off"))
        .and(mode_filter.clone())
        .and(config_filter.clone())
        .and_then(off_http);

    let routes = wave.or(chaser).or(red).or(green).or(blue).or(off);

    warp::serve(routes).run(([0, 0, 0, 0], 80)).await;
}

async fn get_cpu_temp() -> anyhow::Result<usize> {
    let output = Command::new("/bin/cat")
        .arg("/sys/class/thermal/thermal_zone0/temp")
        .output()
        .await?;
    let s = String::from(String::from_utf8(output.stdout)?.trim());
    Ok(s.parse::<usize>()
        .with_context(|| format!("Failed to parse `{}`", s))?)
}

async fn get_gpu_temp() -> anyhow::Result<usize> {
    let output = Command::new("/opt/vc/bin/vcgencmd")
        .arg("measure_temp")
        .output()
        .await?;

    // Strip away non-numbers in `temp=xx.x'C`
    let buffer = output.stdout;
    if buffer.len() < 7 {
        Err(anyhow!("GPU output too small (<7): {:?}", buffer))
    } else {
        let s = String::from(String::from_utf8(buffer)?.trim());
        let f = (&s[5..s.len() - 2])
            .parse::<f32>()
            .with_context(|| format!("Failed to parse `{}`", s))?;
        Ok(f.trunc() as usize)
    }
}

async fn off_http(
    mode: Arc<RwLock<Box<dyn TableMode>>>,
    config: Arc<Config>,
) -> Result<impl warp::Reply, Infallible> {
    let mut mode = mode.write().await;
    *mode = Box::new(Off::new(config.num_lit_leds()));

    Ok("Off")
}

async fn red_http(
    mode: Arc<RwLock<Box<dyn TableMode>>>,
    config: Arc<Config>,
) -> Result<impl warp::Reply, Infallible> {
    let mut mode = mode.write().await;
    *mode = Box::new(Constant::new(
        config.num_lit_leds(),
        Led::new(config.brightness.clone(), 245u8, 68u8, 56u8),
    ));

    Ok("Red")
}

async fn green_http(
    mode: Arc<RwLock<Box<dyn TableMode>>>,
    config: Arc<Config>,
) -> Result<impl warp::Reply, Infallible> {
    let mut mode = mode.write().await;
    *mode = Box::new(Constant::new(
        config.num_lit_leds(),
        Led::new(config.brightness.clone(), 50u8, 245u8, 52u8),
    ));

    Ok("Green")
}

async fn blue_http(
    mode: Arc<RwLock<Box<dyn TableMode>>>,
    config: Arc<Config>,
) -> Result<impl warp::Reply, Infallible> {
    let mut mode = mode.write().await;
    *mode = Box::new(Constant::new(
        config.num_lit_leds(),
        Led::new(config.brightness.clone(), 102u8, 165u8, 242u8),
    ));

    Ok("Blue")
}

async fn wave_http(
    mode: Arc<RwLock<Box<dyn TableMode>>>,
    config: Arc<Config>,
) -> Result<impl warp::Reply, Infallible> {
    let mut mode = mode.write().await;
    *mode = Box::new(Wave::new(config.num_lit_leds(), config.brightness.clone()));

    Ok("Wave")
}

async fn chaser_http(
    mode: Arc<RwLock<Box<dyn TableMode>>>,
    config: Arc<Config>,
) -> Result<impl warp::Reply, Infallible> {
    let mut mode = mode.write().await;
    *mode = Box::new(Chaser::new(Wave::new(
        config.num_lit_leds(),
        config.brightness.clone(),
    )));

    Ok("Chaser")
}
