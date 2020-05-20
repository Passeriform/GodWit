# TODO

☐   Splash
    ☐   Choose godwit config directory(.godwit)
    ☐   Add default organization(default)
    ☐   Add default project(default)
        ☐   Choose directory(/)

☐   Autoscan projects and import settings
    ☐   Recommendations
    ☐   Scan for .gw folders


☐   Track applications
    ☐   Firefox/Chrome

☐   Templates
    ☐   Add option to create settings using template.
    ☐   Add option to create stategraph using template.
    ☐   Add module to check if template is invalid.

✔   Better logger @done(2020-05-15 23:27)
    ✔   Solidify debug logger @done(2020-05-15 23:17)
    ✔   Eliminate println @done(2020-05-15 23:17)
    ✔   Better project list formatting @done(2020-05-15 08:23)
    ☐   Add more debug and info logs

☐   Status operation and modifying status flags
    ☐   Create Settings::status enum and macro

☐   Fix ErrorTypes and match their results
    ☐   Track use-cases

☐   Fix documentation and add examples

☐   Quick Fixes
    ☐   Check out clone() elimination
    ☐   Serialize glyphs differently in serde_json
        ☐   String::into seems like a hack (Implement real serializer)
        ☐   Maybe implement custom serializer/deserializer
    ☐   Complete Core::Status
    ☐   Fix _utils naming problem with fmt
    ☐   Init purge option should not be followed by actual init

✔   Create symlinks if working directory does not equate to default `.godwit` @done(2020-05-19 04:57)
