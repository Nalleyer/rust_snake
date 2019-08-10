# r_snake

## conclusion

弃坑先。

amethyst固然很厉害，前途光明。但是目前看，文档匮乏，学习要深入代码，代价实在有点大。

另一方面，ui和prefab系统虽然都有了，但目前没有工具链，只能手写，加上没有文档，怎么写也不知道。实在痛苦。

因此本实验项目宣告结束。期待amethyst继续发展成熟，到时我再回来研究学习。

## roadmap

* [x] wsad to move the snake
* [x] random food gen
* [x] add a loading state to avoid fast moving on starting
* [x] loading ui
* [ ] snake die and ui stuff


## How to run

To run the game, use

```
cargo run --features "vulkan"
# or simply(with the default feature):
# cargo run
```

on Windows and Linux, and

```
cargo run --features "metal"
```

on macOS.