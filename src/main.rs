use orgize::Org;
use serde_json::to_string_pretty;

const INPUT: &str = r#"
#+title: Joanie's inventions

Some things that Joanie has invented.

* Names
:PROPERTIES:
:CREATED:  [2020-09-23 Wed 18:19]
:END:

+ Miniday the hamster (from Egg Island)
+ Gorga the penguin (actually a duck)
+ Gregor the pirate
+ Linga the llama
+ Lilla the unicorn
+ Quacken the duck
+ Mimonemo the Lion
+ Blooby and Doona, Tony and Boona, Bowie and Dina (other Smeds and Smoos)
+ Baby Geese the blue whale
+ Oinkycha the Pokémon (is the Peppa pig spacehopper)
+ Granny Smith the Panda
+ Grannyboo-Grannybam  (a robot, keeps going forward and backward [Abby scary robot impression])
+ Lad (small sparkly tiger soft toy)
+ Stitch Cuteston
+ Pig the pygmy sloth
+ The otter is very curious, it's called Nosy Rosie
+ The wolf does everything I say, so it's called Obie (or Obe for short)
+ Softy Tumbleshame the Penguin

* Places
:PROPERTIES:
:CREATED:  [2020-09-23 Wed 18:20]
:END:

+ Egg Island
+ Mustard Town

* Things
:PROPERTIES:
:CREATED:  [2020-11-10 Tue 18:02]
:END:

+ *Snowflakers* (shoes for walking through snow and keeping yr feet dry)

* Witches Potion
:PROPERTIES:
:CREATED:  [2021-04-28 Wed 19:17]
:END:

Some Microphones
A bat
Some Witch sweets
Some more witch sweets
A human hand
Two pumpkins
Bubbles
And nothing else

Oh actually
A skeleton head
Your brain
Haha, no not really, you can have your brain back, just needs any brain.

On the subject, the witch's friend Witchy will be going round all the houses putting brains down the chimneys, a bit like Santa does, but for Halloween instead of Christmas.

* New recipe for potion to produce a spoon, in real life, that can talk. Comes in form of witch sweeties.
:PROPERTIES:
:CREATED:  [2021-04-30 Fri 18:01]
:END:

1) A real pumpkin
2) A toy bat
3) A toy pumpkin
4) A microphone
5) All the Rainbow Dash water
6) Bunny water for the bunnies
7)No, bunny water for the bunny! (NOTE the cauldron is a bunny)
8) Parrot water for the bunny.
9) See, he's turned into a parrot now, so more bunny water please ("ah, drink it up now {slurp}")
10) Another pumpkin sweetie please, on the spoon please.
11) I only got one, so can I have some more pumpkin sweeties please.
12) Now mix mix mix.

* Potion water. Makes sparkles come from clouds.
:PROPERTIES:
:CREATED:  [2021-04-30 Fri 18:06]
:END:

1) Sparkle water
2) A poo (don't worry, it won't be pooey!)
3) Ten pumpkins


* A feet potion. Makes toy feet.
:PROPERTIES:
:CREATED:  [2021-04-30 Fri 18:08]
:END:

Only contains feet. Twenty feet. Real human feet. That's a lot of feet. Tastes like feet.

* Messages
:PROPERTIES:
:CREATED:  [2023-06-22 Thu 09:16]
:END:

+ Water Freezey shouting "PUT YOUR HOODY AND UNICORN DRESS ON!" (Note: Water Freezey is a soft toy sheep)
+ Don't forget to draw a mushroom running cheetah fast, with legs, wearing a bridle
"#;


fn main() {
    let parsed_input = Org::parse(INPUT);
    println!("{}", to_string_pretty(&parsed_input).unwrap());
}
