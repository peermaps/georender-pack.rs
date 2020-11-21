## georender-pack.rs

Pack osm data into a buffer based on the [peermaps buffer
schema](https://github.com/peermaps/docs/blob/master/bufferschema.md). This is part of the [peermaps](https://github.com/peermaps/) pipeline.

If you want to decode these buffers, use the [Node.js version of this library](https://github.com/peermaps/georender-pack) or open a pull request here! :tada:

## Usage

For example usage with the [osmpbf](https://crates.io/crates/osmpbf) Rust crate, see
[example/osmpbf/main.rs](example/osmpbf/main.rs).

## Testing

There is a limited test suite on creating and encoding new PeerLine, PeerNode,
and PeerArea objects. It still needs benchmarks for larger osm objects.

```
cargo test
``` 


## Example


```
cargo run --example osmpbf /path/to/my.pbf
```

## License

MIT
