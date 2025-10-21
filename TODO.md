
## TODO for crate
- Go through & build complete, following the documentation approach from slouch
- [ ] Test documentation approach with `cargo doc`

### messages
- [x] Macro for enum types
- [x] Integer types added to parse sequences
- [ ] Finish adding specialized fields & parse (see `nsdq`)
- [ ] Opinionated grammar for type renaming, make note of original
- [x] Consolidate files to match sections in spec, as documentated
`order.rs`
- [x] order cancel/delete
- [ ] order executed
- [ ] order replace
- [ ] Consolidate add types using `Option<Mpid>`? `OrderAdded`?
`stock.rs`
- [ ] luld collar
- [ ] market participant pos
- [ ] mwcb decline
- [ ] mwcb status
- [ ] operational halt
- [ ] quoting period
- [ ] reg sho
- [ ] directory
- [ ] trading action
`system.rs`
- [ ] direct_listing
- [ ] net order imbal
- [ ] system event merged with `EventCode` enum?
- [ ] retail price improvement indicator
`trade.rs`
- [ ] match
- [ ] cross
- [ ] broken


## Future directions
[TCP option](https://www.nasdaqtrader.com/content/technicalsupport/specifications/dataproducts/itchcompression.pdf)


