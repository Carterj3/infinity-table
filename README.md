# Infinity Table

## Assembly Instructions
### Preface
Take the instructions with a grain of salt, the most I _planned_ things out is the referenced SVG. Most of the table was improvised during construction although I had read the below tutorials. Also 2x4 aren't 2in by 4in so the SVG won't actually work (although no wood was wasted since I measured everything and made adjustments).

Additionally I made 2 tables at once so I divided my materials by two to create the parts list here but there could be an issue with doing that.

### Hardware

#### Parts list
The parts list are reflective of the items I used, the exact ones are not linked. The electronics used are under Software.
Again, a local store may have the same parts in a better size, cheaper price, or without shipping cost. If you've never cut acrylic it may be best to have them come correctly sized.
- 50x [Concrete Screws](https://www.homedepot.com/p/Tapcon-3-16-in-x-1-3-4-in-Phillips-Flat-Head-Concrete-Anchors-75-Pack-24355/100136988) -- I use concrete screws because I've never ruin the top of one.
- 1x [24"x34" Clear Acrylic Sheet](https://www.homedepot.com/p/OPTIX-24-in-x-48-in-x-220-in-Acrylic-Sheet-MC-20/202038051) & [Privacy Film](https://www.homedepot.com/p/Gila-3-ft-x-15-ft-Mirror-Privacy-Window-Film-PRS361/100196546) -- I recommend just buying a 2way mirror instead
- 1x [20"x30" Mirror Acrylic Sheet](https://www.homedepot.com/p/Falken-Design-24-in-x-48-in-x-1-8-in-Thick-Acrylic-Mirror-Silver-Sheet-Falken-Design-ACRYLIC-MIR-S-1-8-2448/308669915) -- I used 1/8 inch
- 4x [96" 2by4](https://www.homedepot.com/p/2-in-x-4-in-x-96-in-Prime-Whitewood-Stud-058449/312528776) -- My math says 288" total
- 4x [4ft plywood](https://www.homedepot.com/p/Sanded-Plywood-Common-1-2-in-x-2-ft-x-4-ft-Actual-0-451-in-x-23-75-in-x-47-75-in-300896/202093833) -- I had plywood around so grain of salt here. You might want the midlevel and actual tabletop to be nicer/softer material.
- 1x [Light Switch](https://www.homedepot.com/p/Leviton-15-Amp-Single-Pole-Switch-White-10-Pack-M24-01451-2WM/100075329) & 1x [Light Switch Cover](https://www.homedepot.com/p/Leviton-Decora-1-Gang-Midway-Nylon-Wall-Plate-White-10-Pack-M52-0PJ26-0WM/100356780)
- 1x Quart Black Paint
- 1x Quart Your Choice Paint -- Whatever color you want the table to be.
- 2x [Paint Brushes](https://www.homedepot.com/p/1-in-2-in-and-3-in-Chiseled-Foam-Paint-Brush-Set-9-Pack-A-8509/100128205)
- [M2.5 Screws](https://www.amazon.com/gp/product/B07F14J7X8/ref=ppx_yo_dt_b_asin_title_o07_s00?ie=UTF8&psc=1) -- Used to attach the electronics into the box so get enough.

#### Steps
Keep the [SVG handy](Pictures/Infinity%20Table.svg) to understand what exactly is going on.

The screws almost always need to be sunk when screwed because there ~2" and the wood is >2" so sink them so they can go through ~1" of both pieces. (Basically take a drill with a wider bit than the screw head and drill so it's 1" away from going through and then use the concrete drill bit where the screw will go and then actually insert the screw).

In general, I sanded the parts after I cut them.

##### Construct Half of the legs
Cut 4 pieces of 2x4 17" long. These pieces will run the entire height of the table and support the table top.
Cut 4 pieces of 2x4 5" long. These pieces will support the mid level.
Grab 8 screws.

Drill into the 5" so that the screws can be sunk (the 2x4 is deeper than the screw is long). Then attach the 5" and the 17" together so that there's a "4x4" box looking kinda like a L where the bottom of the L is the two pieces connected and the top part of the L is the rest of the 17" piece.

##### Construct Mid-level
Cut a 4ft plywood to be 24"x42". Cut out from the corners ~2x4 area so that the 17" long legs can go "through" the wood while the 5" legs will support it. This piece will be screwed on top of the 5" legs.

Cut 4 pieces of 2x4 7" long. These pieces will also be screwed in the 17" long rest of the leg above the plywood and will support the tabletop-ish.

##### Construct Top
Cut 2 pieces of 2x4 28" long and 2 pieces of 2x4 45" long then screw them together to make a box with interior dimension of 24"x42". ([Photo of table with exterior box](Pictures/TableOutsideBox.png))

Cut a 4ft plywood to be 27"x45", the extra wood is so that it'll protrude out the sides and can support the box. This sheet of plywood should be screwed into the 7" part of the legs.

Cut 2 pieces of 2x4 24" and 32" this will form a box that fits between all of the legs. Screw this box inside of the other box such that the combined boxes can still fit onto the table. The legs will go up around the inner box but inside the outter box.

Cut 2 4ft plywood to be 27"x45", these two sheets will make up the actual tabletop.
Cut out a 20"x30" hole from the middle of one of the sheet. This is so when you look down you can see the mirror through the glass.
Cut out a 24"x34" hole fromt he middle of the other sheet. This is where the 2way mirror will be inserted and it'll rest on plywood sheet immediate under this one.

Screw through 24"x34" holed plywood into the 20"x30" plywood and finally into the box.

At this point you should be able to put the top box on and off the legs.

On one side of the box you'll need to cut out a rectangular hole for the light switch. Wait to screw it in until you've assembled the electronics.

##### Acyrlics
If you didn't follow my recommendation to have somebody else do the assembly / cutting then cut the Mirror & Clear sheets down to the correct sizes.
Mirror - 20"x30"
Clear - 24"x34"

Cut the privacy screen about 28" long (it's already wide enough). Apply soap-ish water to the clear acyrlic and attach the privacy screen on. Trim of the excess privacy screen but don't worry if you can't get it flush with the acrylic.

Wait until after painting to add these to the table but do make sure they fit.

### Software

#### Parts list
- 1x [Raspi](https://www.canakit.com/raspberry-pi-4-2gb.html?cid=usd&src=raspberrypi) -- Technically I used a v3. An actual microcontroller would be a better fit but I wanted to write Rust.
- 1x [5V power source](https://www.sparkfun.com/products/14602) -- 300 * 1W is bigger than 40W but w/e
- 1x [Wall Plug to cables](https://www.sparkfun.com/products/14603) -- This is to connect to the light switch, you could repo an existing cable and just cut it.
- 1x [APA102 LEDs](https://www.sparkfun.com/products/14016)
- 1x [Screwable Breadboard](https://www.sparkfun.com/products/12070)
- 2x [MOSFET](https://www.sparkfun.com/products/10213) -- Raspi doesn't output 5v so used to convert the 3v to 5v

#### Solder everything
The power source will be connected to the rails of the breadboard.
Connect the V+, GND to the LEDs as well as to Pins 4&6 of the raspi (you can power the raspi via the pins instead of usb).

Connect V+ to each of the 2 MOSFET and then have the output of the MOSFET go to CLK & DATA respectively.
Connect Pin 19 (MOSI) of the Raspi to the DIN MOSFET.
Connect Pin 23 (CLK) of the Raspi to the CLK MOSFT.

Connect the power source to the light switch. If things are connected correctly then turning on the light switch turns on the Pi & after configuring the Pi the leds will also eventually turn on.


#### Configure the Pi
See [Setup](SETUP.md).

#### Sand & Paint the table
Sand the table.

The inside of the boxes should all be black, pretty much anywhere inside of the table is black so that only the LEDs produce light (the rest absorbs it).

Everything else gets painted how you want, keep in mind that the box covers the top of the table so some things may not be visible. Don't forget to paint the underside of the table too.

Put the top on and use a sharpie to trace an outline of the inside box on the table. This is to mark where the mirror goes.

### Combine Hardware & Software

#### Attach electronics
Snake the lightswitch so that it can be screwed correct into the side of the box and then screw it in.

Screw the power supply, breadboard, and raspi into the underside of the table ([See photo of electronics](Pictures/ElectronicsScrew.png)). Use a drill to make a whole first and then you can use a hex tool to easily screw in the M2.5 screws. I found it to be pretty easy if I first put the electronics on the wood and used a sharpie to mark where the hole's are for the drill.

The back of the LEDs was a gluestrip so I just used that to attach them to the box.

#### Attach Acyrlics
Glue the mirror within the traced outline from the sharpie, remove the protective cover of the mirror now.

Add the top the table onto the legs.

Put the "privacy" acrylic within it's holding box on the tabletop, remove the protective cover of the "privacy" acrylic now.

That's it!

## Things I wish I changed
Just buy a 2Way mirror instead of getting clear acrylic + mirror sheet. Although the excess mirror sheet did come in handy for windows.

It'd be cool if I researched at all on how to add outlets to the table. I suspect I could just wire them in parellel with the light switch but I didn't look anything up.

A temp sensor / voltage sensor probably would be useful. Depending on the power source the lights sometimes flicker so then the raspi could detect that and drop the brightness. Also, I didn't do any math, I have no idea how hot the electronics will get so a temp sensor might be nice (Its currently just piggy backing of the raspi's).

The top half of the table shouldn't rest on the plywood and instead should hide it.

Researched more on speeding up the boot time. I tried things like disabling the bluetooth systemd guy and making the pi not require network to boot but I'm pretty sure that actually make it somehow statistically significant slower. So it takes like 17s from flicking the switch to turning on the light, I'd 100% have used one of the esp32's I have if this was a commerical venture.

## Others' tutorials
- https://woodworkjunkie.com/an-infinity-mirror-coffee-table-building-tutorial
- https://www.ikeahackers.net/2018/01/infinity-mirror-coffee-table-ikea-hack.html
