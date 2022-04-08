# GCA

> Generic Contract Architecture for UTXO.

## Work List

- [X] Define trait of wasm runtime.
- [ ] Add wasm runtime support.
    - [X] wasmi
    - [ ] wasmtime
    - [ ] javascript
- [X] Measurer for wasm
    - [X] execute opcode measure
    - [X] memory measure
- [X] RT library for wasm binary
    - [X] env: expose malloc/free, `chain_id` or other function
    - [X] block: access block.
    - [X] tx: access current execute transaction.
    - [X] log: support `log` crate.
    - [X] panic-log: panic info to log.
    - [X] event: emmit event on 
- [ ] Host support for RT library.
- [ ] Testing and Examples.
    - [X] Empty entry function.
    - [X] Log.
    - [ ] Gas measure.
    - [ ] Get chainid.

## Testing

### Dependience

- cargo-make

``` shell
$ cargo make test
```


