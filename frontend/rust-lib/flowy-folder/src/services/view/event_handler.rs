use crate::{
    entities::{
        trash::Trash,
        view::{
            CreateViewParams, CreateViewPayload, RepeatedViewId, UpdateViewParams, UpdateViewPayload, View, ViewId,
        },
    },
    errors::FlowyError,
    services::{TrashController, ViewController},
};
use lib_dispatch::prelude::{data_result, AppData, Data, DataResult};
use std::{convert::TryInto, sync::Arc};

pub(crate) async fn create_view_handler(
    data: Data<CreateViewPayload>,
    controller: AppData<Arc<ViewController>>,
) -> DataResult<View, FlowyError> {
    let params: CreateViewParams = data.into_inner().try_into()?;
    let view = controller.create_view_from_params(params).await?;
    data_result(view)
}

pub(crate) async fn read_view_handler(
    data: Data<ViewId>,
    controller: AppData<Arc<ViewController>>,
) -> DataResult<View, FlowyError> {
    let view_id: ViewId = data.into_inner();
    let mut view = controller.read_view(view_id.clone()).await?;
    // For the moment, app and view can contains lots of views. Reading the view
    // belongings using the view_id.
    view.belongings = controller.read_views_belong_to(&view_id.value).await?;

    data_result(view)
}

#[tracing::instrument(level = "debug", skip(data, controller), err)]
pub(crate) async fn update_view_handler(
    data: Data<UpdateViewPayload>,
    controller: AppData<Arc<ViewController>>,
) -> Result<(), FlowyError> {
    let params: UpdateViewParams = data.into_inner().try_into()?;
    let _ = controller.update_view(params).await?;

    Ok(())
}

pub(crate) async fn delete_view_handler(
    data: Data<RepeatedViewId>,
    view_controller: AppData<Arc<ViewController>>,
    trash_controller: AppData<Arc<TrashController>>,
) -> Result<(), FlowyError> {
    let params: RepeatedViewId = data.into_inner();
    for view_id in &params.items {
        let _ = view_controller.delete_view(view_id.into()).await;
    }

    let trash = view_controller
        .read_local_views(params.items)
        .await?
        .into_iter()
        .map(|view| view.into())
        .collect::<Vec<Trash>>();

    let _ = trash_controller.add(trash).await?;
    Ok(())
}

pub(crate) async fn set_latest_view_handler(
    data: Data<ViewId>,
    controller: AppData<Arc<ViewController>>,
) -> Result<(), FlowyError> {
    let view_id: ViewId = data.into_inner();
    let _ = controller.set_latest_view(&view_id.value)?;
    Ok(())
}

pub(crate) async fn close_view_handler(
    data: Data<ViewId>,
    controller: AppData<Arc<ViewController>>,
) -> Result<(), FlowyError> {
    let view_id: ViewId = data.into_inner();
    let _ = controller.close_view(&view_id.value).await?;
    Ok(())
}

#[tracing::instrument(level = "debug", skip(data, controller), err)]
pub(crate) async fn duplicate_view_handler(
    data: Data<ViewId>,
    controller: AppData<Arc<ViewController>>,
) -> Result<(), FlowyError> {
    let view_id: ViewId = data.into_inner();
    let _ = controller.duplicate_view(&view_id.value).await?;
    Ok(())
}
