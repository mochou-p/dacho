// dacho/examples/usage/src/main.rs

#![expect(clippy::absolute_paths, reason = "example style")]


trait Mesh {
    const VERTEX_COUNT: usize;
    const  INDEX_COUNT: usize;
}

struct Quad;
impl Mesh for Quad {
    const VERTEX_COUNT: usize =  4;
    const  INDEX_COUNT: usize =  6;
}

struct Circle;
impl Mesh for Circle {
    const VERTEX_COUNT: usize =  7;
    const  INDEX_COUNT: usize = 18;
}

struct InstanceData {
    chunk_offset: usize,
    count:        usize
}

#[derive(Debug)]
struct MeshData {
    vertex_offset: usize,
    index_offset:  usize,
    index_count:   usize
}

#[derive(Default)]
struct Meshes {
    registered_meshes:             std::collections::HashMap<String, usize>,
    instance_datas_per_mesh:       std::collections::HashMap<String, (MeshData, Vec<InstanceData>)>,
    current_vertex_offset:         usize,
    current_index_offset:          usize,
    current_instance_chunk_offset: usize,
    vertices:                      Vec<f32>,
    indices:                       Vec<f32>,
    instances:                     Vec<f32>
}

const INSTANCE_SIZE: usize = 2;

impl Meshes {
    fn draw(&self) {
        let mut temp = Vec::with_capacity(self.instance_datas_per_mesh.len());

        for (name, (mesh_data, instance_datas)) in &self.instance_datas_per_mesh {
            temp.push((name, mesh_data));

            let estimate = self.registered_meshes[name];

            for instance_data in instance_datas {
                let  _vertex_offset =    mesh_data.vertex_offset;
                let instance_offset = instance_data.chunk_offset;
                let   _index_offset =     mesh_data.index_offset;

                let    _index_count = mesh_data.index_count;
                let  instance_count =   instance_data.count;

                println!(
                    "{name:<14}({estimate}) : {}\x1b[42m{}\x1b[41m{}\x1b[0m",
                    " ".repeat(instance_offset),
                    " ".repeat(instance_count * INSTANCE_SIZE),
                    " ".repeat((estimate - instance_count) * INSTANCE_SIZE)
                );
            }
        }

        println!("\n{temp:?}");
    }

    fn register<M>(&mut self, count_estimate: usize) {
        let key = std::any::type_name::<M>().to_owned();

        if self.registered_meshes.contains_key(&key) {
            eprintln!("\x1b[33mwarning:\x1b[0m `{key}` registered more than once!`");
        } else {
            self.registered_meshes.insert(key, count_estimate);
        }
    }

    fn add_instance<M: Mesh>(&mut self, instance: [f32; INSTANCE_SIZE]) {
        let key = std::any::type_name::<M>().to_owned();

        let Some(instance_count_estimate) = self.registered_meshes.get(&key) else {
            panic!("\x1b[31merror:\x1b[0m `{key}` has not yet been registered!\n       you need to call `Meshes::register::<T>(..)` before `Meshes::add_instance::<T>(..)`");
        };

        let i = {
            if let Some((_, instance_datas)) = self.instance_datas_per_mesh.get_mut(&key) {
                let instance_data = instance_datas.last_mut().unwrap();

                if instance_data.count == *instance_count_estimate {
                    println!("\x1b[46;30manother\x1b[0m chunk for {key}");

                    let new_chunk = InstanceData {
                        chunk_offset: self.current_instance_chunk_offset,
                        count:        1
                    };
                    let i = new_chunk.chunk_offset;

                    self.instances.resize(self.instances.len() + (instance_count_estimate * INSTANCE_SIZE), 0.0);
                    instance_datas.push(new_chunk);

                    self.current_instance_chunk_offset += instance_count_estimate * INSTANCE_SIZE;

                    i
                } else {
                    println!("\x1b[43;30mlast\x1b[0m chunk for {key}");

                    let i = instance_data.chunk_offset + (instance_data.count * INSTANCE_SIZE);

                    instance_data.count += 1;

                    i
                }
            } else {
                println!("\x1b[42;30mfirst\x1b[0m chunk for {key}");

                let instance_data = InstanceData {
                    chunk_offset: self.current_instance_chunk_offset,
                    count:        1
                };
                let i = instance_data.chunk_offset;

                let mesh_data = MeshData {
                    vertex_offset: self.current_vertex_offset,
                    index_offset:  self.current_index_offset,
                    index_count:   M::INDEX_COUNT
                };

                self. vertices.extend(vec![0.0; M::VERTEX_COUNT]);
                self.  indices.extend(vec![0.0; M:: INDEX_COUNT]);
                self.instances.resize(self.instances.len() + (instance_count_estimate * INSTANCE_SIZE), 0.0);
                self.instance_datas_per_mesh.insert(key, (mesh_data, vec![instance_data]));

                self.current_vertex_offset         += M::VERTEX_COUNT;
                self.current_index_offset          += M:: INDEX_COUNT;
                self.current_instance_chunk_offset += instance_count_estimate * INSTANCE_SIZE;

                i
            }
        };

        let range = i..i + INSTANCE_SIZE;
        println!("range={range:?}\n");
        self.instances.splice(range, instance);
    }
}

fn main() {
    let mut meshes = Meshes::default();

    meshes.register::<Quad>  (3);
    meshes.register::<Circle>(7);

    meshes.add_instance::<Circle>([    1.0; INSTANCE_SIZE]);
    meshes.add_instance::<Quad>  ([90001.0; INSTANCE_SIZE]);
    meshes.add_instance::<Quad>  ([90002.0; INSTANCE_SIZE]);
    meshes.add_instance::<Quad>  ([90003.0; INSTANCE_SIZE]);
    meshes.add_instance::<Quad>  ([90004.0; INSTANCE_SIZE]);
    meshes.add_instance::<Quad>  ([90005.0; INSTANCE_SIZE]);
    meshes.add_instance::<Quad>  ([90006.0; INSTANCE_SIZE]);
    meshes.add_instance::<Quad>  ([90007.0; INSTANCE_SIZE]);
    meshes.add_instance::<Circle>([    2.0; INSTANCE_SIZE]);
    meshes.add_instance::<Circle>([    3.0; INSTANCE_SIZE]);
    meshes.add_instance::<Circle>([    4.0; INSTANCE_SIZE]);
    meshes.add_instance::<Circle>([    5.0; INSTANCE_SIZE]);
    meshes.add_instance::<Circle>([    6.0; INSTANCE_SIZE]);
    meshes.add_instance::<Circle>([    7.0; INSTANCE_SIZE]);
    meshes.add_instance::<Circle>([    8.0; INSTANCE_SIZE]);
    meshes.add_instance::<Circle>([    9.0; INSTANCE_SIZE]);
    meshes.add_instance::<Circle>([   10.0; INSTANCE_SIZE]);
    meshes.add_instance::<Circle>([   11.0; INSTANCE_SIZE]);
    meshes.add_instance::<Circle>([   12.0; INSTANCE_SIZE]);
    meshes.add_instance::<Circle>([   13.0; INSTANCE_SIZE]);
    meshes.add_instance::<Circle>([   14.0; INSTANCE_SIZE]);
    meshes.add_instance::<Circle>([   15.0; INSTANCE_SIZE]);
    meshes.add_instance::<Circle>([   16.0; INSTANCE_SIZE]);
    meshes.add_instance::<Quad>  ([90008.0; INSTANCE_SIZE]);
    meshes.add_instance::<Quad>  ([90009.0; INSTANCE_SIZE]);
    meshes.add_instance::<Quad>  ([90010.0; INSTANCE_SIZE]);
    meshes.add_instance::<Circle>([   17.0; INSTANCE_SIZE]);
    meshes.add_instance::<Quad>  ([90011.0; INSTANCE_SIZE]);
    meshes.add_instance::<Quad>  ([90012.0; INSTANCE_SIZE]);
    meshes.add_instance::<Circle>([   18.0; INSTANCE_SIZE]);
    meshes.add_instance::<Circle>([   19.0; INSTANCE_SIZE]);
    meshes.add_instance::<Circle>([   20.0; INSTANCE_SIZE]);
    meshes.add_instance::<Circle>([   21.0; INSTANCE_SIZE]);
    meshes.add_instance::<Circle>([   22.0; INSTANCE_SIZE]);
    meshes.add_instance::<Quad>  ([90013.0; INSTANCE_SIZE]);

    meshes.draw();

    println!("\n{:?}", meshes.instances);

    // dacho::window::main();
}

