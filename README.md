CLI application utilizing the [unifac crate](https://github.com/sarcaustech/unifac).

# Setup
`unifac` crate is used as local dependency. Therefore, both projects are assumed to be located in the same parent directory. Crate is referenced with `../unifac`. Installation is done via `cargo install --path <PATH TO CRATE>`

# Usage (PoC)
Program takes a YAML file as its input. YAML structure is excected as:
```yaml
temperature: 298 # temperature in Kelvin
substances: # array
  ethanole: # substance name (used only in the result for readability)
    fraction: 0.5 # fraction of substance in mixture
    groups: # array of functional groups in current substance
      - "1:1" # formatted as "<GROUP ID>:<COUNT OF GROUP IN SUBSTANCE>"
      - "2:1"
      - "14:1"
  benzene:
    fraction: 0.5
    groups:
      - "9:6"
```
For group IDs, see [used data source](http://www.ddbst.com/published-parameters-unifac.html).
The program itself can be called using:
```unifac-cli [OPTIONS] <YAML-FILE>```
The output will be printed to stdout, unless a output file is set with `-o`.
For more help on these options use `--help`.
