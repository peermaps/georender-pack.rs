## georender-pack.rs

Pack osm data into a buffer based on the [peermaps buffer
schema](https://github.com/peermaps/docs/blob/master/bufferschema.md). This is part of the [peermaps](https://github.com/peermaps/) pipeline.

If you want to decode these buffers, use the [Node.js version of this library](https://github.com/peermaps/georender-pack) or open a pull request here! :tada:


## Usage

### `encode::node`

```rust
encode::node(
    id: u64, 
    point: (f64, f64), 
    tags: Vec<(&str, &str)>
) -> Result<Vec<u8>, Error> 
```

```rust
use georender_pack::encode;

let id = 1831881213;
let lon = 12.253938100000001;
let lat = 54.09006660000001;
let tags = vec![("name", "Neu Broderstorf"), ("traffic_sign", "city_limit")];

let bytes = encode::node(id (lon, lat), &tags).unwrap();
```


### `encode::way`

```rust
encode::way(
    id: u64, 
    tags: Vec<(&str, &str)>,
    refs: Vec<i64>,
    deps: &HashMap<i64, (f64, f64)>
) -> Result<Vec<u8>, Error> 
```

```rust
use georender_pack::encode;

let tags = vec![("source", "bing"), ("highway", "residential")];
let refs = vec![1, 5, 3];
let mut deps = HashMap::new();
deps.insert(1, (31.184799400000003, 29.897739500000004));
deps.insert(5, (31.184888100000002, 29.898801400000004));
deps.insert(3, (31.184858400000003, 29.8983899));
let bytes = encode::way(234941233, tags, refs, &deps).unwrap();
```

## Example

For example usage with the [osmpbf](https://crates.io/crates/osmpbf) Rust crate for parsing PBF files, see
[example/osmpbf/main.rs](example/osmpbf/main.rs).


```
cargo run --example osmpbf /path/to/my.pbf
```

## Development

```
cargo test
```

There is a limited test suite on creating and encoding new PeerLine, PeerNode,
and PeerArea objects. 

## License

MIT
