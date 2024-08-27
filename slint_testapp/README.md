# Sandbox Slint App

This is a demo app for myself using the official `slint` template as a start,
as I like the file structure it provides. 

## Features implements

- A tab widgets with various tabs and program elements on there
- The counter on one page that is by default implemented in the template
- A simple page with buttons that raise dialogs via `rtd`. Note: on Linux the `gtk3` dev package must be installed, see `rtd` docs for more info.
- A settings dialog that is opened via a settings button on the main page and has an editable spinbox, accepted on ok and discarded on cancel. The main page has a `print` button that prints the current value of the spinbox.

## What slint cannot do at this point but would be nice

- Disable the main UI when the settings dialog is opened (see [this issue](https://github.com/slint-ui/slint/issues/2338))
  - One workaround could be to hide the UI, see messagebox example on the dialog pane
- Menubar, statusbar, toolbar, etc. (see, e.g., [this issue](https://github.com/slint-ui/slint/issues/38))
