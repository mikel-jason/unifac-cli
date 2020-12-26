CLI application utilizing the [unifac crate](https://github.com/sarcaustech/unifac).

# Setup
`unifac` crate is used as local dependency. Therefore, both projects are assumed to be located in the same parent directory. Crate is referenced with `../unifac`.

# Usage (PoC)
Either edit [assets/demo.yaml](assets/demo.yaml) or add own file and change file path in [src/main.rs](src/main.rs). YAML structure is excected as:
```yaml
temperature: 298 # temperature in Kelvin
substances: # array
  ethanole: # substance name (not used, only for readability)
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