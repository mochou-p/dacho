// dacho/examples/usage/src/main.rs

#![expect(clippy::absolute_paths, reason = "example style")]


const   VERTEX_SIZE: usize = 2;
const    INDEX_SIZE: usize = 3;
const INSTANCE_SIZE: usize = 2;

trait Mesh {
    fn vertices() -> &'static [[f32; VERTEX_SIZE]];
    fn  indices() -> &'static [[u32;  INDEX_SIZE]];
}

struct Quad;
impl Mesh for Quad {
    fn vertices() -> &'static [[f32; VERTEX_SIZE]] {
        &[
            [-0.1, -0.1],
            [-0.1,  0.0],
            [ 0.1, -0.1],
            [ 0.1,  0.1]
        ]
    }

    fn indices() -> &'static [[u32; INDEX_SIZE]] {
        &[
            [0, 1, 2],
            [2, 1, 3]
        ]
    }
}

struct Circle;
impl Mesh for Circle {
    fn vertices() -> &'static [[f32; VERTEX_SIZE]] {
        &[
            [ 0.0, -0.2], //      0
            [ 0.0,  0.0], //
            [ 0.1,  0.1], // 6         2
            [ 0.1, -0.1], //      1
            [ 0.0,  0.2], // 5         3
            [-0.1, -0.2], //
            [-0.1,  0.2]  //      4
        ]
    }

    fn indices() -> &'static [[u32; INDEX_SIZE]] {
        &[
            [0, 1, 2],
            [2, 1, 3],
            [3, 1, 4],
            [4, 1, 5],
            [5, 1, 6],
            [6, 1, 0]
        ]
    }
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

impl Meshes {
    fn with_size_estimates(
        different_meshes_count: usize,
        vertex_buffer_size:     usize,
        index_buffer_size:      usize,
        instance_buffer_size:   usize
    ) -> Self {
        Self {
            registered_meshes: std::collections::HashMap::with_capacity(different_meshes_count),
            vertices:                                Vec::with_capacity(    vertex_buffer_size),
            indices:                                 Vec::with_capacity(     index_buffer_size),
            instances:                               Vec::with_capacity(  instance_buffer_size),
            ..Default::default()
        }
    }

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

    fn register<M: Mesh>(&mut self, instance_count_estimate: usize) {
        let key = std::any::type_name::<M>().to_owned();

        debug_assert!(!self.registered_meshes.contains_key(&key), "mesh already registered");

        self.registered_meshes.insert(key, instance_count_estimate);
    }

    fn add_instance<M: Mesh>(&mut self, instance: [f32; INSTANCE_SIZE]) {
        let key = std::any::type_name::<M>().to_owned();

        let Some(instance_count_estimate) = self.registered_meshes.get(&key) else {
            panic!("\x1b[31merror:\x1b[0m `{key}` has not yet been registered!\n       you need to call `Meshes::register::<T>(..)` before `Meshes::add_instance::<T>(..)`");
        };
        let estimated_size = instance_count_estimate * INSTANCE_SIZE;

        let i = {
            if let Some((_, instance_datas)) = self.instance_datas_per_mesh.get_mut(&key) {
                let instance_data = instance_datas.last_mut().unwrap();

                if instance_data.count == *instance_count_estimate {
                    // another chunk for M

                    let new_chunk = InstanceData {
                        chunk_offset: self.current_instance_chunk_offset,
                        count:        1
                    };
                    let i = new_chunk.chunk_offset;

                    self.instances.resize(self.instances.len() + estimated_size, 0.0);
                    instance_datas.push(new_chunk);

                    self.current_instance_chunk_offset += estimated_size;

                    i
                } else {
                    // last chunk for M

                    let i = instance_data.chunk_offset + (instance_data.count * INSTANCE_SIZE);

                    instance_data.count += 1;

                    i
                }
            } else {
                // first chunk for M

                let instance_data = InstanceData {
                    chunk_offset: self.current_instance_chunk_offset,
                    count:        1
                };
                let i = instance_data.chunk_offset;

                let     vertices = M::vertices();
                let      indices = M:: indices();
                let vertex_count = vertices.len() * VERTEX_SIZE;
                let  index_count =  indices.len() *  INDEX_SIZE;

                let mesh_data = MeshData {
                    vertex_offset: self.current_vertex_offset,
                    index_offset:  self.current_index_offset,
                    index_count
                };

                // TODO: move to Self::register
                self.vertices.extend(unsafe {
                    std::slice::from_raw_parts(vertices.as_ptr() as *const f32, vertex_count)
                });
                self.indices .extend(unsafe {
                    std::slice::from_raw_parts( indices.as_ptr() as *const f32,  index_count)
                });

                self.instances.resize(self.instances.len() + estimated_size, 0.0);

                self.instance_datas_per_mesh.insert(key, (mesh_data, vec![instance_data]));

                self.current_vertex_offset         +=    vertex_count;
                self.current_index_offset          +=     index_count;
                self.current_instance_chunk_offset += estimated_size;

                i
            }
        };

        self.instances.splice(i..i + INSTANCE_SIZE, instance);
    }
}

fn main() {
    let mut meshes = Meshes::with_size_estimates(2, 32, 32, 128);

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

