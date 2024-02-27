# [LIGNUM] 🪵

#### ⟿ A simple [boids algorithm](https://en.wikipedia.org/wiki/Boids) implementation

## Origin

Looked cool, so I wanted to do it myself, at first I wanted to do it in OpenGL,

but then it meant that I should learn every concept in graphics programming and I have no such time (for now)

Thus I made it with a rust game engine.

It works great (though it's highly unoptimize 🤫), but as I took some freedom in the process, I _kind of_ deviated from

boids original algorithm, as a consequence it's more a fluid simulation game (with bird?) than bird flying in a flock

but who care, I had fun and I could watch all day these birds simulating a fluid.

## Purpose

1. Have fun
2. To understand some gamedev concept
3. Improving my rust skills

## Installation

> If you don't want to bother yourself compiling this, you can check out the [web demo](https://ilingu.github.io/lignum)
>
> Beware that it's a really simplified version (a demo...) of the actual algorithm (e.g: birds sprite/animation and the user interface cannot be load in web, the perfomance are not really great either)

For the true ~performant~ app with real birds 🐦 you'll have to compile this yourself with cargo

## Made with:

1. **Elegance** ✅
2. `RUST` ✨🦀
3. [ggez](https://github.com/ggez/ggez): _game engine_
