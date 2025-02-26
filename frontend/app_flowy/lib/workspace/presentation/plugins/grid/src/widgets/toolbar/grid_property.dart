import 'package:app_flowy/startup/startup.dart';
import 'package:app_flowy/workspace/application/grid/field/field_service.dart';
import 'package:app_flowy/workspace/application/grid/setting/property_bloc.dart';
import 'package:app_flowy/workspace/presentation/plugins/grid/src/layout/sizes.dart';
import 'package:app_flowy/workspace/presentation/plugins/grid/src/widgets/header/field_editor.dart';
import 'package:app_flowy/workspace/presentation/plugins/grid/src/widgets/header/field_type_extension.dart';
import 'package:flowy_infra/image.dart';
import 'package:flowy_infra/theme.dart';
import 'package:flowy_infra_ui/flowy_infra_ui.dart';
import 'package:flowy_infra_ui/style_widget/button.dart';
import 'package:flowy_infra_ui/style_widget/icon_button.dart';
import 'package:flowy_infra_ui/style_widget/text.dart';
import 'package:flowy_infra_ui/widget/spacing.dart';
import 'package:flowy_sdk/protobuf/flowy-grid-data-model/grid.pb.dart' show Field;
import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:styled_widget/styled_widget.dart';

class GridPropertyList extends StatelessWidget with FlowyOverlayDelegate {
  final String gridId;
  final List<Field> fields;
  const GridPropertyList({
    required this.gridId,
    required this.fields,
    Key? key,
  }) : super(key: key);

  void show(BuildContext context) {
    FlowyOverlay.of(context).insertWithAnchor(
      widget: OverlayContainer(
        child: this,
        constraints: BoxConstraints.loose(const Size(260, 400)),
      ),
      identifier: identifier(),
      anchorContext: context,
      anchorDirection: AnchorDirection.bottomRight,
      style: FlowyOverlayStyle(blur: false),
      delegate: this,
    );
  }

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create: (context) =>
          getIt<GridPropertyBloc>(param1: gridId, param2: fields)..add(const GridPropertyEvent.initial()),
      child: BlocBuilder<GridPropertyBloc, GridPropertyState>(
        builder: (context, state) {
          final cells = state.fields.map((field) {
            return _GridPropertyCell(gridId: gridId, field: field);
          }).toList();

          return ListView.separated(
            shrinkWrap: true,
            controller: ScrollController(),
            separatorBuilder: (context, index) {
              return VSpace(GridSize.typeOptionSeparatorHeight);
            },
            itemCount: cells.length,
            itemBuilder: (BuildContext context, int index) {
              return cells[index];
            },
          );
        },
      ),
    );
  }

  String identifier() {
    return toString();
  }

  @override
  bool asBarrier() => true;
}

class _GridPropertyCell extends StatelessWidget {
  final Field field;
  final String gridId;
  const _GridPropertyCell({required this.gridId, required this.field, Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    final theme = context.watch<AppTheme>();

    final checkmark = field.visibility
        ? svgWidget('home/show', color: theme.iconColor)
        : svgWidget('home/hide', color: theme.iconColor);

    return Row(
      children: [
        Expanded(
          child: SizedBox(
            height: GridSize.typeOptionItemHeight,
            child: FlowyButton(
              text: FlowyText.medium(field.name, fontSize: 12),
              hoverColor: theme.hover,
              leftIcon: svgWidget(field.fieldType.iconName(), color: theme.iconColor),
              onTap: () {
                FieldEditor(
                  gridId: gridId,
                  fieldContextLoader: FieldContextLoaderAdaptor(gridId: gridId, field: field),
                ).show(context, anchorDirection: AnchorDirection.bottomRight);
              },
            ),
          ),
        ),
        FlowyIconButton(
          hoverColor: theme.hover,
          width: GridSize.typeOptionItemHeight,
          onPressed: () {
            context.read<GridPropertyBloc>().add(GridPropertyEvent.setFieldVisibility(field.id, !field.visibility));
          },
          icon: checkmark.padding(all: 6),
        )
      ],
    );
  }
}
