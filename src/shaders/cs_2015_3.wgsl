@group(0)
@binding(0)
var<storage, read> input: array<u32>;

struct HousePos {
    north: u32,
    east: u32,
    index: u32,
};

@group(0)
@binding(1)
var<storage, read_write> pos: HousePos; 

@group(0)
@binding(2)
var tex: texture_storage_2d<rgba8unorm, read_write>;


@compute
@workgroup_size(1)
fn main(@builtin(global_invocation_id) id: vec3<u32>) {
    var color = textureLoad(tex, vec2<i32>(i32(pos.east), i32(pos.north)));
    if color.x >= 1.0 {
        color.y += 0.2;
    } else if color.y >= 1.0 {
        color.z += 0.2;
    } else {
        color.x += 0.2;
    }
    textureStore(tex, vec2<i32>(i32(pos.east), i32(pos.north)), color);

    let inp = input[pos.index];
    if inp == 60u {
        pos.east -= 1u;
    } else if inp == 62u {
        pos.east += 1u;
    } else if inp == 94u {
        pos.north -= 1u;
    } else if inp == 118u {
        pos.north += 1u;
    }

    pos.index = pos.index + 1u;
} 
