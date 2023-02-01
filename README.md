# jack-midi-tools
Jack Midi client applications to use for music creation. It started as a hobby project where the goal is to use midi events to alter visual animations.


## Packages

### Jack midi logger

### Jack module

`jack-module` is a rust library for creating jack modules. Hiding most of the complexity so module developers can concentrate on what they want to implement.

### Midi device

`midi-device` is a rust library to keep track of the state of a midi device.

## Midi events

`midi-events` is a rust library that contains data types for parsing and using midi.

### Music notes

`music-notes` is a rust library containing data types for music in general. It has been setup to be not specific to western music only. Although currently only a generic layer and the chromatic scale have been implemented it can be easily extended to support other scales as well.

### EGUI Widgets for music

`egui-widgets-music` is a rust library containing widgets to be used with `egui`. The widgets are music related.

#### Piano Keys

Widget to draw a piano. Has options to use different colors when keys are pressed.

![Piano Keys](resources/egui-piano-keys.png "egui-widgets-music::piano_keys")
