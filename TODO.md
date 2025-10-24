
## TODO for crate
- Go through & build complete, following the documentation approach from slouch
- [ ] Test documentation approach with `cargo doc`

### messages
- [x] Finish adding specialized fields & parse (see `nsdq-util`)
- [x] Opinionated grammar for type renaming, make note of original
- [ ] Consolidate types where possible?
`order.rs`
- [x] order executed
- [x] order replace
`stock.rs`
- [x] luld collar
- [x] market participant pos
- [x] mwcb decline
- [x] mwcb status
- [x] operational halt
- [x] quoting period
- [x] reg sho
- [ ] directory (`IssueSubType`)
- [ ] trading action (`ActionReason`) (Generator function for strings)
`system.rs`
- [x] direct_listing
- [x] net order imbal
- [x] system event merged with `EventCode` enum?
- [x] retail price improvement indicator
`trade.rs`
- [x] match
- [x] cross
- [x] broken


## Future directions
[TCP option](https://www.nasdaqtrader.com/content/technicalsupport/specifications/dataproducts/itchcompression.pdf)


