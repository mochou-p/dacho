// dacho/tests/ecs.rs

#![expect(clippy::allow_attributes_without_reason)]

#[cfg(test)]
#[expect(clippy::should_panic_without_expect)]
#[expect(clippy::undocumented_unsafe_blocks)]
#[expect(clippy::unwrap_used)]
#[expect(clippy::let_underscore_must_use)]
#[expect(clippy::let_underscore_untyped)]
mod tests {
    use dacho::{App, Query, system};


    // explicit type makes `Query` understand its a ZST
    type Nothing = ();

    mod app {
        use super::*;

        #[test]
        #[should_panic]
        fn empty_app_panics() {
            let app = App::new("");

            app.run();
        }

        #[test]
        fn app_runs() {
            let mut app = App::new("");
            app.no_window_run_once(true);

            #[system]
            fn system(__: Query<Nothing>) {}

            app.insert(system);

            app.run();
        }

        #[test]
        fn no_duplication() {
            let mut app = App::new("");
            app.no_window_run_once(true);

            app.world.spawn(((),));

            #[system]
            fn system_1(query: Query<Nothing>) {
                assert_eq!(query.iter().count(), 1);
            }

            #[system]
            fn system_2(__: Query<Nothing>) {}

            app.insert(system_1);
            app.insert(system_2);

            app.run();
        }
    }

    mod system {
        use super::*;

        #[test]
        fn system_runs() {
            static mut CALLED: bool = false;

            let mut app = App::new("");
            app.no_window_run_once(true);

            app.world.spawn(((),));

            #[system]
            fn system(__: Query<Nothing>) {
                unsafe { CALLED = true; }
            }

            app.insert(system);

            app.run();

            assert!(unsafe { CALLED });
        }
    }

    mod query {
        use super::*;

        #[test]
        fn first() {
            let mut app = App::new("");
            app.no_window_run_once(true);

            app.world.spawn((1_u8,));
            app.world.spawn((2_u8,));

            #[system]
            fn system(query: Query<u8>) {
                assert_eq!(*query.first(), 1);
            }

            app.insert(system);

            app.run();
        }

        #[test]
        fn first_mut() {
            let mut app = App::new("");
            app.no_window_run_once(true);

            app.world.spawn((1_u8,));
            app.world.spawn((3_u8,));

            #[system]
            fn system_1(query: Query<u8>) {
                *unsafe { query.first_mut() } *= 2;
            }

            #[system]
            fn system_2(query: Query<u8>) {
                assert_eq!(*query.first(), 2);
            }

            app.insert(system_1);
            app.insert(system_2);

            app.run();
        }

        #[test]
        fn iter() {
            let mut app = App::new("");
            app.no_window_run_once(true);

            app.world.spawn((1_u8,));
            app.world.spawn((2_u8,));

            #[system]
            fn system(query: Query<u8>) {
                let mut sum = 0;

                for number in query.iter() {
                    sum += number;
                }

                assert_eq!(sum, 3);
            }

            app.insert(system);

            app.run();
        }

        #[test]
        fn iter_mut() {
            let mut app = App::new("");
            app.no_window_run_once(true);

            app.world.spawn((1_u8,));
            app.world.spawn((2_u8,));

            #[system]
            fn system_1(query: Query<u8>) {
                for number in unsafe { query.iter_mut() } {
                    *number *= 2;
                }
            }

            #[system]
            fn system_2(query: Query<u8>) {
                let mut sum = 0;

                for number in query.iter() {
                    sum += number;
                }

                assert_eq!(sum, 6);
            }

            app.insert(system_1);
            app.insert(system_2);

            app.run();
        }

        #[test]
        fn entities_iter() {
            let mut app = App::new("");
            app.no_window_run_once(true);

            app.world.spawn((1_u8, 2_u8));
            app.world.spawn((3_u8,));

            #[system]
            fn system(query: Query<u8>) {
                assert_eq!(
                    query.entities_iter()
                        .map(|entity| entity.iter_unchecked::<u8>().sum::<u8>())
                        .sum::<u8>(),
                    6
                );
            }

            app.insert(system);

            app.run();
        }

        #[test]
        fn entities_iter_mut() {
            use dacho::entity::Entity;

            let mut app = App::new("");
            app.no_window_run_once(true);

            app.world.spawn((1_u8, 2_u8));
            app.world.spawn((3_u8,));

            #[system]
            fn system_1(query: Query<u8>) {
                let numbers = unsafe { query.entities_iter_mut() }
                    .flat_map(Entity::iter_mut_unchecked::<u8>);

                for n in numbers {
                    *n *= 2;
                }
            }

            #[system]
            fn system_2(query: Query<u8>) {
                assert_eq!(
                    query.entities_iter()
                        .map(|entity| entity.iter_unchecked::<u8>().sum::<u8>())
                        .sum::<u8>(),
                    12
                );
            }

            app.insert(system_1);
            app.insert(system_2);

            app.run();
        }
    }

    mod entity {
        use super::*;

        #[test]
        fn has() {
            let mut app = App::new("");
            app.no_window_run_once(true);

            app.world.spawn((true,));

            #[system]
            fn system(query: Query<bool>) {
                let entity = query.entity_first();

                assert!( entity.has::<bool>());
                assert!(!entity.has::<u128>());
            }

            app.insert(system);

            app.run();
        }

        #[test]
        fn count() {
            let mut app = App::new("");
            app.no_window_run_once(true);

            app.world.spawn((true, false, true));

            #[system]
            fn system(query: Query<bool>) {
                let entity = query.entity_first();

                assert_eq!(entity.count::<bool>(), 3);
                assert_eq!(entity.count::<u128>(), 0);
            }

            app.insert(system);

            app.run();
        }

        #[test]
        fn first() {
            let mut app = App::new("");
            app.no_window_run_once(true);

            app.world.spawn((20_u8,));

            #[system]
            fn system(query: Query<u8>) {
                let entity = query.entity_first();

                assert_eq!(*entity.first::<u8>().unwrap(), 20);
                assert_eq!( entity.first::<i8>(),          None);
            }

            app.insert(system);

            app.run();
        }

        #[test]
        fn first_unchecked() {
            let mut app = App::new("");
            app.no_window_run_once(true);

            app.world.spawn((20_u8,));

            #[system]
            fn system(query: Query<u8>) {
                let entity = query.entity_first();

                assert_eq!(*entity.first_unchecked::<u8>(), 20);
            }

            app.insert(system);

            app.run();
        }

        #[test]
        #[should_panic]
        fn first_unchecked_panic() {
            let mut app = App::new("");
            app.no_window_run_once(true);

            app.world.spawn((20_u8,));

            #[system]
            fn system(query: Query<u8>) {
                let entity = query.entity_first();

                let _ = entity.first_unchecked::<i8>();
            }

            app.insert(system);

            app.run();
        }

        #[test]
        fn first_mut() {
            let mut app = App::new("");
            app.no_window_run_once(true);

            app.world.spawn((20_u8,));

            #[system]
            fn system_1(query: Query<u8>) {
                let entity = unsafe { query.entity_first_mut() };

                *entity.first_mut::<u8>().unwrap() = 30;
            }

            #[system]
            fn system_2(query: Query<u8>) {
                let entity = query.entity_first();

                assert_eq!(*entity.first::<u8>().unwrap(), 30);
            }

            app.insert(system_1);
            app.insert(system_2);

            app.run();
        }

        #[test]
        fn first_mut_unchecked() {
            let mut app = App::new("");
            app.no_window_run_once(true);

            app.world.spawn((20_u8,));

            #[system]
            fn system_1(query: Query<u8>) {
                let entity = unsafe { query.entity_first_mut() };

                *entity.first_mut_unchecked::<u8>() = 30;
            }

            #[system]
            fn system_2(query: Query<u8>) {
                let entity = query.entity_first();

                assert_eq!(*entity.first_unchecked::<u8>(), 30);
            }

            app.insert(system_1);
            app.insert(system_2);

            app.run();
        }

        #[test]
        #[should_panic]
        fn first_mut_unchecked_panic() {
            let mut app = App::new("");
            app.no_window_run_once(true);

            app.world.spawn((20_u8,));

            #[system]
            fn system(query: Query<u8>) {
                let entity = unsafe { query.entity_first_mut() };

                let _ = entity.first_mut_unchecked::<i8>();
            }

            app.insert(system);

            app.run();
        }

        #[test]
        fn iter() {
            let mut app = App::new("");
            app.no_window_run_once(true);

            app.world.spawn((1_u8, 2_u8));

            #[system]
            fn system(query: Query<u8>) {
                let entity = query.entity_first();

                assert_eq!(entity.iter::<u8>().unwrap().sum::<u8>(), 3);
            }

            app.insert(system);

            app.run();
        }

        #[test]
        fn iter_unchecked() {
            let mut app = App::new("");
            app.no_window_run_once(true);

            app.world.spawn((1_u8, 2_u8));

            #[system]
            fn system(query: Query<u8>) {
                let entity = query.entity_first();

                assert_eq!(entity.iter_unchecked::<u8>().sum::<u8>(), 3);
            }

            app.insert(system);

            app.run();
        }

        #[test]
        #[should_panic]
        fn iter_unchecked_panic() {
            let mut app = App::new("");
            app.no_window_run_once(true);

            app.world.spawn((1_u8, 2_u8));

            #[system]
            fn system(query: Query<u8>) {
                let entity = query.entity_first();

                let _ = entity.iter_unchecked::<i8>();
            }

            app.insert(system);

            app.run();
        }

        #[test]
        fn iter_mut() {
            let mut app = App::new("");
            app.no_window_run_once(true);

            app.world.spawn((1_u8, 2_u8));

            #[system]
            fn system_1(query: Query<u8>) {
                let entity = unsafe { query.entity_first_mut() };

                for x in entity.iter_mut::<u8>().unwrap() {
                    *x *= 2;
                }
            }

            #[system]
            fn system_2(query: Query<u8>) {
                let entity = query.entity_first();

                assert_eq!(entity.iter::<u8>().unwrap().sum::<u8>(), 6);
            }

            app.insert(system_1);
            app.insert(system_2);

            app.run();
        }

        #[test]
        fn iter_mut_unchecked() {
            let mut app = App::new("");
            app.no_window_run_once(true);

            app.world.spawn((1_u8, 2_u8));

            #[system]
            fn system_1(query: Query<u8>) {
                let entity = unsafe { query.entity_first_mut() };

                for x in entity.iter_mut_unchecked::<u8>() {
                    *x *= 2;
                }
            }

            #[system]
            fn system_2(query: Query<u8>) {
                let entity = query.entity_first();

                assert_eq!(entity.iter_unchecked::<u8>().sum::<u8>(), 6);
            }

            app.insert(system_1);
            app.insert(system_2);

            app.run();
        }

        #[test]
        #[should_panic]
        fn iter_mut_unchecked_panic() {
            let mut app = App::new("");
            app.no_window_run_once(true);

            app.world.spawn((1_u8, 2_u8));

            #[system]
            fn system(query: Query<u8>) {
                let entity = unsafe { query.entity_first_mut() };

                let _ = entity.iter_mut_unchecked::<i8>();
            }

            app.insert(system);

            app.run();
        }
    }
}

