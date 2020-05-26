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

☐   Add more debug, trace and info logs

☐   Status operation and modifying status flags
    ☐   Create Settings::status enum and macro

☐   Match errorkind to their output message
    ☐   Track use-cases
    ☐   Use errortypes instead of Box<Error>

☐   Add examples to docs

☐   Serialize glyphs differently in serde_json
    ☐   String::into seems like a hack (Implement real serializer)
    ☐   Maybe implement custom serializer/deserializer

☐   Quick Fixes
    ☐   Complete Core::Status
    ☐   Fix _utils naming problem with fmt
    ☐   Modify function to accept generics in traits
    ☐   Convert pathbuf to paths in function and try using join

☐   Tests

☐   Create symlinks if working directory does not equate to default `.godwit`
