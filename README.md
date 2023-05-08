# Swift Localized Generator

<strong>Swift Localized Generator</strong> generates a `Swift` enum `LocalizationKey` that enumerates all the defined localizable strings in the `Localizable.strings` file.

### How to use

1. Install Rust on your system. You can download Rust from the official website:<br>https://www.rust-lang.org/tools/install</li>

2. Clone this repository to your local machine.

3. Build the rust project for release:
    ```
    cd swift_localized_generator
    cargo build --release
    ```
    
4. Run the `swift_localized_generator` executable passing in the path to the `Localizable.strings` file and the path to the `Swift` file where the generated enum should be written to:
    ```
    ./target/release/swift_localized_generator /path/to/Localizable.strings /path/to/LocalizationKeys.swift
    ```
    
The `LocalizationKey` enum will be generated in the specified `Swift` file and can be used to reference localizable strings in your code.

Note: Make sure to add the generated Swift file to your Xcode project to avoid build errors.

### Xcode integration

You can add <strong>Swift Localized Generator</strong> to your project's `Build Phases`.

To add the Swift Localized Generator executable to an Xcode build phase, follow these steps:

1. Open your Xcode project.

2. Select your project in the project navigator.

3. Select the target you want to add the build phase to.

4. Click on the "Build Phases" tab.

5. Click on the "+" button to add a new build phase.

6. Select "New Run Script Phase" from the dropdown.

7. In the script editor, enter the following command to run the Swift Localized Generator executable, passing in the paths to the `Localizable.strings` file and the Swift file where the generated enum should be written to:

    ```
    /path/to/swift_localized_generator /path/to/Localizable.strings /path/to/LocalizationKey.swift
    ```
Replace `/path/to/swift_localized_generator` with the path to the `swift_localized_generator` executable that you built earlier, and replace `/path/to/Localizable.strings` and `/path/to/LocalizationKey.swift` with the appropriate paths for your project.

8. Make sure that the script is set to be run before the "Compile Sources" phase.

9. Build your project to generate the `LocalizationKey` enum in the specified Swift file.

Note: If the `swift_localized_generator` executable is not in your project directory, you will need to provide the absolute path to the executable in the build phase script. Also, make sure that the script file has execute permissions. You can set the execute permission by running the following command in Terminal:

```
chmod +x /path/to/build_phase_script.sh
```

Replace `/path/to/build_phase_script.sh` with the path to the build phase script.






