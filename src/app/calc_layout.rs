use super::{ContainerForSend, DefaultLayout, LayoutForSend, WorkspaceForSend};
use fxhash::FxHashMap;

#[derive(PartialEq, Eq, Debug)]
pub struct WorkspaceWithLayout(FxHashMap<ContainerForSend, Rect>);

/// 左上の角を`(0, 0)`、右下の角を`(255, 255)`とするWorkspaceの中のWindowの形
#[derive(PartialEq, Eq, Debug)]
pub struct Rect {
    left_top: (u8, u8),
    right_bottom: (u8, u8),
}

impl TryFrom<&WorkspaceForSend> for WorkspaceWithLayout {
    type Error = anyhow::Error;

    fn try_from(value: &WorkspaceForSend) -> Result<Self, Self::Error> {
        match &value.layout {
            LayoutForSend::Default(layout) => match layout {
                DefaultLayout::BSP => default_layout::bsp(&value.items),
                DefaultLayout::Columns => todo!(),
                DefaultLayout::Rows => todo!(),
                DefaultLayout::VerticalStack => todo!(),
                DefaultLayout::HorizontalStack => todo!(),
                DefaultLayout::UltrawideVerticalStack => todo!(),
                DefaultLayout::Grid => todo!(),
                DefaultLayout::RightMainVerticalStack => todo!(),
            },
        }
    }
}

mod default_layout {
    use anyhow::{bail, Context as _, Ok, Result};
    use fxhash::FxHashMap;

    use crate::app::{calc_layout::Rect, ContainerForSend};

    use super::WorkspaceWithLayout;

    const MAX: u8 = 255u8;

    pub fn bsp(items: &Vec<ContainerForSend>) -> Result<WorkspaceWithLayout> {
        todo!()
    }
    pub fn colums(items: &Vec<ContainerForSend>) -> Result<WorkspaceWithLayout> {
        let item_count: u8 = items.len().try_into()?;
        let weight_per_item = dbg!(MAX / item_count);
        let mut hashmap: FxHashMap<ContainerForSend, Rect> = FxHashMap::default();
        for i in 0..item_count {
            let tmp = weight_per_item * i;
            let top_x_i = weight_per_item * (i + 1);
            let rect = Rect {
                left_top: (tmp, 0),
                right_bottom: (top_x_i, 255),
            };
            hashmap.insert(items[i as usize].clone(), rect);
        }
        Ok(WorkspaceWithLayout(hashmap))
    }
    /// columsのxとyを入れ替える
    pub fn rows(items: &Vec<ContainerForSend>) -> Result<WorkspaceWithLayout> {
        let columns = colums(items)?;
        let mut hashmap: FxHashMap<ContainerForSend, Rect> = FxHashMap::default();
        for (k, v) in columns.0 {
            let c_left_top = v.left_top;
            let c_right_bottom = v.right_bottom;
            let rect = Rect {
                left_top: (c_left_top.1, c_left_top.0),
                right_bottom: (c_right_bottom.1, c_right_bottom.0),
            };
            hashmap.insert(k, rect);
        }
        Ok(WorkspaceWithLayout(hashmap))
    }
    pub fn vertical_stack(items: &Vec<ContainerForSend>) -> Result<WorkspaceWithLayout> {
        todo!()
    }
    pub fn horizontal_stack(items: &Vec<ContainerForSend>) -> Result<WorkspaceWithLayout> {
        todo!()
    }
    pub fn ultrawide_vertical_stack(items: &Vec<ContainerForSend>) -> Result<WorkspaceWithLayout> {
        todo!()
    }
    pub fn grid(items: &Vec<ContainerForSend>) -> Result<WorkspaceWithLayout> {
        todo!()
    }
    pub fn right_main_vertical_stack(items: &Vec<ContainerForSend>) -> Result<WorkspaceWithLayout> {
        todo!()
    }
    #[cfg(test)]
    mod tests {

        use rand::Rng as _;

        use super::*;

        fn make_mock_container() -> ContainerForSend {
            ContainerForSend {
                windows: vec![crate::app::WindowForSend {
                    id: rand::thread_rng().r#gen(),
                    icon: crate::app::Icon::default(),
                    accent_color: 0x000000,
                }],
            }
        }

        fn make_mock_containers(num_items: u8) -> Vec<ContainerForSend> {
            (0..num_items)
                .into_iter()
                .map(|_| make_mock_container())
                .collect()
        }

        #[test]
        fn test_row_two_containers() -> Result<()> {
            let layout_rects = rows(&make_mock_containers(2))?
                .0
                .into_values()
                .collect::<Vec<Rect>>();
            assert_eq!(
                layout_rects,
                vec![
                    Rect {
                        left_top: (0, 0),
                        right_bottom: (255, 127)
                    },
                    Rect {
                        left_top: (0, 127),
                        right_bottom: (255, 254)
                    }
                ]
            );
            Ok(())
        }

        #[test]
        fn test_colums_two_containers() -> Result<()> {
            let layout_rects = colums(&make_mock_containers(2))?
                .0
                .into_values()
                .collect::<Vec<Rect>>();
            assert_eq!(
                layout_rects,
                vec![
                    Rect {
                        left_top: (0, 0),
                        right_bottom: (127, 255)
                    },
                    Rect {
                        left_top: (127, 0),
                        right_bottom: (254, 255)
                    }
                ]
            );
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {}
