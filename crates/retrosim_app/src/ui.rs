//! Native Bevy UI. The world uses integer image scaling; interface type has its own scale.
use bevy::prelude::*;
use retrosim_sim::{Order, Status};

use crate::{Lab, Mode, Tool, UiAction, Visuals};

const INK: Color = Color::srgb(0.035, 0.063, 0.065);
const PANEL: Color = Color::srgb(0.067, 0.106, 0.108);
const RAISED: Color = Color::srgb(0.094, 0.145, 0.145);
const LINE: Color = Color::srgb(0.20, 0.27, 0.25);
const GOLD: Color = Color::srgb(0.86, 0.71, 0.42);
const PAPER: Color = Color::srgb(0.87, 0.90, 0.82);
const MUTED: Color = Color::srgb(0.57, 0.67, 0.61);
const JADE: Color = Color::srgb(0.35, 0.76, 0.57);
const RED: Color = Color::srgb(0.87, 0.43, 0.34);

#[derive(Resource, Default)]
pub struct ScreenLayout {
    pub world: Rect,
    pub width: f32,
    pub height: f32,
}

#[derive(Component)]
pub struct UiRoot;

#[derive(Component, Clone, Copy)]
pub enum Label {
    Time,
    Beat,
    Party(usize),
    PartyActivity(usize),
    Objective,
    Selected,
    SelectedInfo,
    SelectedIntent,
    PreviewTitle,
    PreviewDetail,
    PreviewNumbers,
    Confirm,
    Pause,
    Log,
    Resources,
    Toast,
    ResultTitle,
    ResultDetail,
    Hover,
}

#[derive(Component)]
pub struct HealthBar(pub usize);

#[derive(Component)]
pub struct SelectedPortrait;

#[derive(Component, Clone, Copy)]
pub enum Overlay {
    Help,
    Result,
    Toast,
}

fn absolute(x: f32, y: f32, w: f32, h: f32) -> Node {
    Node {
        position_type: PositionType::Absolute,
        left: px(x),
        top: px(y),
        width: px(w),
        height: px(h),
        flex_shrink: 0.0,
        ..default()
    }
}

fn attach(commands: &mut Commands, parent: Entity, child: Entity) -> Entity {
    commands.entity(parent).add_child(child);
    child
}

fn panel(commands: &mut Commands, parent: Entity, node: Node, fill: Color) -> Entity {
    let mut node = node;
    node.border = UiRect::all(px(1));
    node.border_radius = BorderRadius::all(px(3));
    let child = commands
        .spawn((node, BackgroundColor(fill), BorderColor::all(LINE)))
        .id();
    attach(commands, parent, child)
}

fn text_at(
    commands: &mut Commands,
    parent: Entity,
    node: Node,
    value: &str,
    size: f32,
    color: Color,
    label: Option<Label>,
) -> Entity {
    let child = commands
        .spawn((
            node,
            Text::new(value),
            TextFont {
                font_size: size,
                ..default()
            },
            TextColor(color),
        ))
        .id();
    if let Some(label) = label {
        commands.entity(child).insert(label);
    }
    attach(commands, parent, child)
}

fn image_at(commands: &mut Commands, parent: Entity, node: Node, image: Handle<Image>) -> Entity {
    let child = commands.spawn((node, ImageNode::new(image))).id();
    attach(commands, parent, child)
}

fn button(
    commands: &mut Commands,
    parent: Entity,
    node: Node,
    value: &str,
    size: f32,
    action: UiAction,
    label: Option<Label>,
) -> Entity {
    let width = match node.width {
        Val::Px(value) => value,
        _ => 100.0,
    };
    let node = Node {
        align_items: AlignItems::Center,
        justify_content: JustifyContent::Center,
        ..node
    };
    let child = panel(commands, parent, node, RAISED);
    commands.entity(child).insert((Button, action));
    text_at(
        commands,
        child,
        Node {
            width: px(width - 8.0),
            margin: UiRect::horizontal(px(4.0)),
            ..default()
        },
        value,
        size,
        PAPER,
        label,
    );
    child
}

/// Rebuilds only when the window dimensions change. Coordinates are logical pixels;
/// main.rs sets the window scale factor override to one for exact mouse mapping.
pub fn layout_ui(
    mut commands: Commands,
    window: Single<&Window>,
    visuals: Res<Visuals>,
    mut layout: ResMut<ScreenLayout>,
    roots: Query<Entity, With<UiRoot>>,
) {
    let width = window.width();
    let height = window.height();
    if width == layout.width && height == layout.height && !roots.is_empty() {
        return;
    }
    for root in &roots {
        commands.entity(root).despawn();
    }
    let s = (width / 1600.0).clamp(1.0, 1.6);
    let margin = 12.0 * s;
    let sidebar = 286.0 * s;
    let gap = 14.0 * s;
    let left_width = width - 2.0 * margin - sidebar - gap;
    let party_y = margin + 50.0 * s;
    let world_top = party_y + 42.0 * s + 8.0 * s;
    let footer_y = height - margin - 18.0 * s - 6.0 * s - 64.0 * s;
    let world_height = footer_y - world_top - 8.0 * s;
    let zoom = (left_width / 440.0)
        .min(world_height / 240.0)
        .floor()
        .max(1.0);
    let scene_w = 440.0 * zoom;
    let scene_h = 240.0 * zoom;
    let scene_x = margin + (left_width - scene_w) / 2.0;
    let scene_y = world_top + (world_height - scene_h) / 2.0;
    layout.width = width;
    layout.height = height;
    layout.world = Rect::from_corners(
        Vec2::new(scene_x, scene_y),
        Vec2::new(scene_x + scene_w, scene_y + scene_h),
    );

    let root = commands
        .spawn((
            UiRoot,
            absolute(0.0, 0.0, width, height),
            BackgroundColor(INK),
        ))
        .id();
    text_at(
        &mut commands,
        root,
        absolute(margin, margin, 204.0 * s, 26.0 * s),
        "RETROSIM",
        24.0 * s,
        GOLD,
        None,
    );
    text_at(
        &mut commands,
        root,
        absolute(margin, margin + 29.0 * s, 205.0 * s, 18.0 * s),
        "COMBAT COMPARISON LAB",
        10.0 * s,
        MUTED,
        None,
    );
    button(
        &mut commands,
        root,
        absolute(margin + 213.0 * s, margin + 3.0 * s, 179.0 * s, 36.0 * s),
        "A  /  World waits",
        14.0 * s,
        UiAction::Mode(Mode::Step),
        None,
    );
    button(
        &mut commands,
        root,
        absolute(margin + 400.0 * s, margin + 3.0 * s, 179.0 * s, 36.0 * s),
        "B  /  Real-time + pause",
        13.0 * s,
        UiAction::Mode(Mode::Realtime),
        None,
    );
    text_at(
        &mut commands,
        root,
        absolute(margin + 593.0 * s, margin + 12.0 * s, 228.0 * s, 25.0 * s),
        "WORLD STOPPED",
        14.0 * s,
        JADE,
        Some(Label::Time),
    );
    let side_x = width - margin - sidebar;
    button(
        &mut commands,
        root,
        absolute(side_x, margin + 3.0 * s, 88.0 * s, 36.0 * s),
        "Reset  R",
        13.0 * s,
        UiAction::Reset,
        None,
    );
    button(
        &mut commands,
        root,
        absolute(side_x + 96.0 * s, margin + 3.0 * s, 107.0 * s, 36.0 * s),
        "Variation",
        13.0 * s,
        UiAction::Variation,
        None,
    );
    button(
        &mut commands,
        root,
        absolute(side_x + 211.0 * s, margin + 3.0 * s, 75.0 * s, 36.0 * s),
        "Help  F1",
        12.0 * s,
        UiAction::Help,
        None,
    );

    let card_gap = 8.0 * s;
    let card_w = (left_width - 2.0 * card_gap) / 3.0;
    for id in 0..3 {
        let card = panel(
            &mut commands,
            root,
            absolute(
                margin + id as f32 * (card_w + card_gap),
                party_y,
                card_w,
                42.0 * s,
            ),
            PANEL,
        );
        commands
            .entity(card)
            .insert((Button, UiAction::SelectActor(id)));
        image_at(
            &mut commands,
            card,
            absolute(4.0 * s, 4.0 * s, 32.0 * s, 32.0 * s),
            visuals.portraits[id].clone(),
        );
        text_at(
            &mut commands,
            card,
            absolute(44.0 * s, 3.0 * s, card_w - 51.0 * s, 17.0 * s),
            "",
            12.0 * s,
            PAPER,
            Some(Label::Party(id)),
        );
        let track = panel(
            &mut commands,
            card,
            absolute(44.0 * s, 22.0 * s, card_w - 52.0 * s, 3.0 * s),
            INK,
        );
        let fill = commands
            .spawn((
                Node {
                    width: percent(100),
                    height: percent(100),
                    ..default()
                },
                BackgroundColor(JADE),
                HealthBar(id),
            ))
            .id();
        attach(&mut commands, track, fill);
        text_at(
            &mut commands,
            card,
            absolute(44.0 * s, 28.0 * s, card_w - 51.0 * s, 12.0 * s),
            "",
            10.0 * s,
            MUTED,
            Some(Label::PartyActivity(id)),
        );
    }
    text_at(
        &mut commands,
        root,
        absolute(side_x, party_y + 9.0 * s, sidebar, 23.0 * s),
        "",
        13.0 * s,
        MUTED,
        Some(Label::Beat),
    );

    let stage = panel(
        &mut commands,
        root,
        absolute(margin, world_top, left_width, world_height),
        Color::srgb(0.026, 0.050, 0.046),
    );
    image_at(
        &mut commands,
        stage,
        absolute(scene_x - margin, scene_y - world_top, scene_w, scene_h),
        visuals.scene.clone(),
    );
    // The image has no Button or Interaction component: world clicks go through the controller.
    let badge = panel(
        &mut commands,
        stage,
        absolute(9.0 * s, 9.0 * s, 208.0 * s, 27.0 * s),
        PANEL,
    );
    text_at(
        &mut commands,
        badge,
        absolute(9.0 * s, 5.0 * s, 193.0 * s, 20.0 * s),
        "TRAIL CLEARING  /  DEMO",
        11.0 * s,
        GOLD,
        None,
    );
    text_at(
        &mut commands,
        stage,
        absolute(
            12.0 * s,
            world_height - 25.0 * s,
            left_width - 160.0 * s,
            19.0 * s,
        ),
        "",
        11.0 * s,
        PAPER,
        Some(Label::Hover),
    );
    button(
        &mut commands,
        stage,
        absolute(
            left_width - 117.0 * s,
            world_height - 34.0 * s,
            105.0 * s,
            25.0 * s,
        ),
        "Grid  G",
        11.0 * s,
        UiAction::Grid,
        None,
    );

    let side = panel(
        &mut commands,
        root,
        absolute(side_x, world_top, sidebar, world_height),
        PANEL,
    );
    let inset = 12.0 * s;
    let inner = sidebar - 24.0 * s;
    text_at(
        &mut commands,
        side,
        absolute(inset, 10.0 * s, inner, 18.0 * s),
        "THE DISCOVERY",
        13.0 * s,
        GOLD,
        None,
    );
    text_at(
        &mut commands,
        side,
        absolute(inset, 33.0 * s, inner, 40.0 * s),
        "",
        13.0 * s,
        PAPER,
        Some(Label::Objective),
    );
    let selected_y = 83.0 * s;
    let portrait = image_at(
        &mut commands,
        side,
        absolute(inset, selected_y, 48.0 * s, 48.0 * s),
        visuals.portraits[0].clone(),
    );
    commands.entity(portrait).insert(SelectedPortrait);
    text_at(
        &mut commands,
        side,
        absolute(70.0 * s, selected_y, inner - 58.0 * s, 20.0 * s),
        "",
        15.0 * s,
        PAPER,
        Some(Label::Selected),
    );
    text_at(
        &mut commands,
        side,
        absolute(70.0 * s, selected_y + 25.0 * s, inner - 58.0 * s, 18.0 * s),
        "",
        12.0 * s,
        JADE,
        Some(Label::SelectedInfo),
    );
    text_at(
        &mut commands,
        side,
        absolute(inset, selected_y + 57.0 * s, inner, 35.0 * s),
        "",
        12.0 * s,
        MUTED,
        Some(Label::SelectedIntent),
    );

    let preview_y = 180.0 * s;
    let preview = panel(
        &mut commands,
        side,
        absolute(8.0 * s, preview_y, sidebar - 16.0 * s, 122.0 * s),
        INK,
    );
    text_at(
        &mut commands,
        preview,
        absolute(9.0 * s, 8.0 * s, inner, 18.0 * s),
        "",
        14.0 * s,
        GOLD,
        Some(Label::PreviewTitle),
    );
    text_at(
        &mut commands,
        preview,
        absolute(9.0 * s, 31.0 * s, inner, 39.0 * s),
        "",
        12.0 * s,
        PAPER,
        Some(Label::PreviewDetail),
    );
    text_at(
        &mut commands,
        preview,
        absolute(9.0 * s, 73.0 * s, inner, 17.0 * s),
        "",
        11.0 * s,
        MUTED,
        Some(Label::PreviewNumbers),
    );
    button(
        &mut commands,
        preview,
        absolute(9.0 * s, 94.0 * s, 162.0 * s, 23.0 * s),
        "Confirm  Enter",
        12.0 * s,
        UiAction::Confirm,
        Some(Label::Confirm),
    );
    button(
        &mut commands,
        preview,
        absolute(178.0 * s, 94.0 * s, 81.0 * s, 23.0 * s),
        "Cancel  Esc",
        10.5 * s,
        UiAction::Cancel,
        None,
    );

    text_at(
        &mut commands,
        side,
        absolute(inset, 314.0 * s, inner, 17.0 * s),
        "COMPANION ORDERS",
        11.0 * s,
        GOLD,
        None,
    );
    let order_width = (inner - 6.0 * s) / 2.0;
    for (index, (name, order)) in [
        ("Protect", Order::Protect),
        ("Focus target", Order::Focus),
        ("Regroup", Order::Regroup),
        ("Withdraw", Order::Withdraw),
    ]
    .into_iter()
    .enumerate()
    {
        button(
            &mut commands,
            side,
            absolute(
                inset + (index % 2) as f32 * (order_width + 6.0 * s),
                336.0 * s + (index / 2) as f32 * 29.0 * s,
                order_width,
                25.0 * s,
            ),
            name,
            11.5 * s,
            UiAction::Order(order),
            None,
        );
    }
    text_at(
        &mut commands,
        side,
        absolute(inset, 401.0 * s, inner, 16.0 * s),
        "RECENT EVENTS",
        11.0 * s,
        GOLD,
        None,
    );
    text_at(
        &mut commands,
        side,
        absolute(
            inset,
            423.0 * s,
            inner,
            (world_height - 431.0 * s).max(35.0 * s),
        ),
        "",
        11.0 * s,
        MUTED,
        Some(Label::Log),
    );

    let ability_w = (left_width - 5.0 * 8.0 * s) / 6.0;
    let tools = [
        Tool::Strike,
        Tool::Bolt,
        Tool::Guard,
        Tool::Heal,
        Tool::Interact,
        Tool::Wait,
    ];
    let names = [
        "1  Strike",
        "2  Bolt",
        "3  Guard",
        "4  Potion",
        "E  Interact",
        ".  Wait",
    ];
    let notes = [
        "5 damage / adjacent",
        "4 damage / 2 focus",
        "Block 3 this beat",
        "Restore 8 health",
        "Investigate / gather",
        "Recover 1 focus",
    ];
    for id in 0..6 {
        let tile = panel(
            &mut commands,
            root,
            absolute(
                margin + id as f32 * (ability_w + 8.0 * s),
                footer_y,
                ability_w,
                64.0 * s,
            ),
            PANEL,
        );
        commands
            .entity(tile)
            .insert((Button, UiAction::SelectTool(tools[id])));
        image_at(
            &mut commands,
            tile,
            absolute(8.0 * s, 9.0 * s, 24.0 * s, 24.0 * s),
            visuals.icons[id].clone(),
        );
        text_at(
            &mut commands,
            tile,
            absolute(38.0 * s, 10.0 * s, ability_w - 43.0 * s, 22.0 * s),
            names[id],
            13.0 * s,
            PAPER,
            None,
        );
        text_at(
            &mut commands,
            tile,
            absolute(8.0 * s, 43.0 * s, ability_w - 12.0 * s, 15.0 * s),
            notes[id],
            10.0 * s,
            MUTED,
            None,
        );
    }
    let footer = panel(
        &mut commands,
        root,
        absolute(side_x, footer_y, sidebar, 64.0 * s),
        PANEL,
    );
    text_at(
        &mut commands,
        footer,
        absolute(10.0 * s, 8.0 * s, inner, 18.0 * s),
        "",
        12.0 * s,
        JADE,
        Some(Label::Resources),
    );
    button(
        &mut commands,
        footer,
        absolute(9.0 * s, 34.0 * s, sidebar - 18.0 * s, 25.0 * s),
        "",
        12.0 * s,
        UiAction::Pause,
        Some(Label::Pause),
    );
    text_at(
        &mut commands,
        root,
        absolute(
            margin,
            height - margin - 17.0 * s,
            width - 2.0 * margin,
            17.0 * s,
        ),
        "WASD / arrows  Move     Click  Inspect / aim     Enter  Confirm     Space  Wait / pause     Esc  Cancel     F1  Controls",
        11.0 * s,
        MUTED,
        None,
    );

    let toast_w = (610.0 * s).min(left_width - 24.0 * s);
    let toast = panel(
        &mut commands,
        root,
        absolute(
            margin + (left_width - toast_w) / 2.0,
            footer_y - 48.0 * s,
            toast_w,
            35.0 * s,
        ),
        RAISED,
    );
    commands
        .entity(toast)
        .insert((Overlay::Toast, GlobalZIndex(20)));
    text_at(
        &mut commands,
        toast,
        absolute(12.0 * s, 8.0 * s, toast_w - 24.0 * s, 25.0 * s),
        "",
        12.0 * s,
        PAPER,
        Some(Label::Toast),
    );

    let help = commands
        .spawn((
            absolute(0.0, 0.0, width, height),
            BackgroundColor(Color::srgba(0.015, 0.025, 0.025, 0.94)),
            GlobalZIndex(40),
            Overlay::Help,
        ))
        .id();
    attach(&mut commands, root, help);
    let help_w = (690.0 * s).min(width - 48.0 * s);
    let help_h = (490.0 * s).min(height - 48.0 * s);
    let help_card = panel(
        &mut commands,
        help,
        absolute(
            (width - help_w) / 2.0,
            (height - help_h) / 2.0,
            help_w,
            help_h,
        ),
        PANEL,
    );
    text_at(
        &mut commands,
        help_card,
        absolute(25.0 * s, 23.0 * s, help_w - 50.0 * s, 35.0 * s),
        "A world that moves with you",
        24.0 * s,
        GOLD,
        None,
    );
    text_at(
        &mut commands,
        help_card,
        absolute(25.0 * s, 72.0 * s, help_w - 50.0 * s, 74.0 * s),
        "Reach the sealed marker, investigate it, then return to the western exit with the discovery. Your two companions act on their own. All names and encounter content are demo placeholders.",
        15.0 * s,
        PAPER,
        None,
    );
    text_at(
        &mut commands,
        help_card,
        absolute(25.0 * s, 155.0 * s, help_w - 50.0 * s, 72.0 * s),
        "A / World waits: each move or confirmed ability advances one beat.\nB / Real-time + pause: Space starts or stops the clock. Confirmed actions queue for the next beat; companions keep acting while time runs.",
        14.0 * s,
        JADE,
        None,
    );
    text_at(
        &mut commands,
        help_card,
        absolute(25.0 * s, 238.0 * s, help_w - 50.0 * s, 167.0 * s),
        "WASD / arrows     Move one square\nClick a character     Inspect health and known intent\n1 / 2 / 3 / 4 / E / .     Strike / bolt / guard / potion / interact / wait\nEnter     Confirm the visible action\nSpace     Wait in A; pause or resume in B\nEsc     Clear a target or queued action / close help\nR     Reset this encounter       G     Toggle square grid\nCompanion orders     Protect / focus target / regroup / withdraw",
        14.0 * s,
        PAPER,
        None,
    );
    button(
        &mut commands,
        help_card,
        absolute(25.0 * s, help_h - 58.0 * s, help_w - 50.0 * s, 34.0 * s),
        "Return to the clearing  /  F1 or Esc",
        14.0 * s,
        UiAction::CloseHelp,
        None,
    );

    let result = commands
        .spawn((
            absolute(0.0, 0.0, width, height),
            BackgroundColor(Color::srgba(0.015, 0.025, 0.025, 0.86)),
            GlobalZIndex(30),
            Overlay::Result,
        ))
        .id();
    attach(&mut commands, root, result);
    let result_w = (590.0 * s).min(width - 48.0 * s);
    let result_h = 236.0 * s;
    let result_card = panel(
        &mut commands,
        result,
        absolute(
            (width - result_w) / 2.0,
            (height - result_h) / 2.0,
            result_w,
            result_h,
        ),
        PANEL,
    );
    text_at(
        &mut commands,
        result_card,
        absolute(25.0 * s, 25.0 * s, result_w - 50.0 * s, 40.0 * s),
        "",
        27.0 * s,
        GOLD,
        Some(Label::ResultTitle),
    );
    text_at(
        &mut commands,
        result_card,
        absolute(25.0 * s, 82.0 * s, result_w - 50.0 * s, 70.0 * s),
        "",
        15.0 * s,
        PAPER,
        Some(Label::ResultDetail),
    );
    let result_button_w = (result_w - 66.0 * s) / 3.0;
    button(
        &mut commands,
        result_card,
        absolute(25.0 * s, 174.0 * s, result_button_w, 36.0 * s),
        "Try again  R",
        13.0 * s,
        UiAction::Reset,
        None,
    );
    button(
        &mut commands,
        result_card,
        absolute(
            33.0 * s + result_button_w,
            174.0 * s,
            result_button_w,
            36.0 * s,
        ),
        "A / World waits",
        12.0 * s,
        UiAction::Mode(Mode::Step),
        None,
    );
    button(
        &mut commands,
        result_card,
        absolute(
            41.0 * s + result_button_w * 2.0,
            174.0 * s,
            result_button_w,
            36.0 * s,
        ),
        "B / Real-time",
        12.0 * s,
        UiAction::Mode(Mode::Realtime),
        None,
    );
}

/// All label and style updates use disjoint query filters to avoid conflicting Node access.
pub fn update_ui(
    lab: Res<Lab>,
    visuals: Res<Visuals>,
    layout: Res<ScreenLayout>,
    mut labels: Query<(&Label, &mut Text, &mut TextColor)>,
    mut bars: Query<(&HealthBar, &mut Node), Without<Overlay>>,
    mut overlays: Query<(&Overlay, &mut Node), Without<HealthBar>>,
    mut portraits: Query<&mut ImageNode, With<SelectedPortrait>>,
    mut buttons: Query<(
        &UiAction,
        &Interaction,
        &mut BackgroundColor,
        &mut BorderColor,
    )>,
) {
    let state = &lab.state;
    let player = state.actor(0);
    let selected = state.actor(lab.selected);
    let action = lab.pending.as_ref().or(lab.queued.as_ref());
    let preview = action.map(|action| state.preview(action));
    let s = (layout.width / 1600.0).clamp(1.0, 1.6);
    let recent_lines = ((layout.height - 620.0 * s) / (34.0 * s))
        .floor()
        .clamp(2.0, 10.0) as usize;
    for (label, mut text, mut color) in &mut labels {
        let value = match *label {
            Label::Time => {
                color.0 = if lab.mode == Mode::Realtime && !lab.paused { GOLD } else { JADE };
                if state.status != Status::Active { "ENCOUNTER COMPLETE".into() }
                else if lab.mode == Mode::Step { "WORLD WAITS FOR YOU".into() }
                else if lab.paused { "PAUSED  /  SPACE TO RUN".into() }
                else { "TIME RUNNING".into() }
            }
            Label::Beat => format!("Beat {:03}  /  Scenario {:03}", state.beat, state.seed),
            Label::Party(id) => { let actor=state.actor(id); format!("{}  {}/{} HP", actor.label, actor.hp, actor.max_hp) }
            Label::PartyActivity(id) => { let actor=state.actor(id); if actor.hp <= 0 { "DOWN / cannot act".into() } else { actor.activity.clone() } }
            Label::Objective => if state.discovery { "Discovery recovered. Return to the western exit.".into() } else { "Investigate the sealed marker. Leave with its discovery.".into() },
            Label::Selected => selected.label.to_owned(),
            Label::SelectedInfo => format!("{} / {} HP  |  {} tiles", selected.hp, selected.max_hp, (selected.x-player.x).abs()+(selected.y-player.y).abs()),
            Label::SelectedIntent => if selected.hp <= 0 { "Down. No action this beat.".into() } else { format!("Intent: {}", selected.intent.text) },
            Label::PreviewTitle => if lab.pending.is_none() && lab.queued.is_some() { "ACTION QUEUED".into() } else if let Some(preview)=&preview { preview.title.clone() } else { "CHOOSE YOUR NEXT ACTION".into() },
            Label::PreviewDetail => {
                color.0 = if preview.as_ref().is_some_and(|p| !p.ok) { RED } else { PAPER };
                if let Some(preview)=&preview { preview.detail.clone() } else { "Click a threat or destination. Pick an ability below to see its effect.".into() }
            }
            Label::PreviewNumbers => if let Some(preview)=&preview { format!("Cost {}  /  Range {}  /  Effect {}", preview.cost, preview.range, preview.damage) } else { "No action selected / no focus spent".into() },
            Label::Confirm => if lab.pending.is_some() { if lab.mode==Mode::Realtime { "Queue action  Enter".into() } else { "Confirm  Enter".into() } } else if lab.queued.is_some() { "Queued / Space to run".into() } else { "Select a target".into() },
            Label::Pause => if lab.mode==Mode::Step { "World waits  /  Space: wait".into() } else if lab.paused { "Resume time  /  Space".into() } else { "Pause time  /  Space".into() },
            Label::Log => state.log.iter().rev().take(recent_lines).map(|entry| format!("{:02}  {}",entry.beat,entry.text)).collect::<Vec<_>>().join("\n"),
            Label::Resources => format!("Focus {}/{}  /  Potions {}  /  Herbs {}", player.energy, player.max_energy, state.potions, state.herbs),
            Label::Toast => lab.notice.clone(),
            Label::ResultTitle => match state.status { Status::Won=>"Discovery secured", Status::Retreated=>"The party withdrew", Status::Defeated=>"The party was overcome", Status::Active=>"" }.into(),
            Label::ResultDetail => match state.status { Status::Won=>"The marker has been investigated and the party returned safely. Which pace made you want another encounter?", Status::Retreated=>"The party left the clearing without the discovery. Reset freely, or compare the other pace.", Status::Defeated=>"This encounter can be reset freely. Try pausing to inspect the threats, or issue a companion order.", Status::Active=>"" }.into(),
            Label::Hover => lab.hover.map(|(x,y)| format!("Square {}, {}  /  Click to aim or move",x,y)).unwrap_or_else(|| "You control the adventurer. Companions act autonomously.".into()),
        };
        if text.0 != value {
            text.0 = value;
        }
    }
    for (bar, mut node) in &mut bars {
        let actor = state.actor(bar.0);
        node.width = percent(100.0 * (actor.hp.max(0) as f32 / actor.max_hp as f32));
    }
    for (kind, mut node) in &mut overlays {
        let show = match kind {
            Overlay::Help => lab.help,
            Overlay::Result => state.status != Status::Active,
            Overlay::Toast => lab.clock < lab.notice_until && !lab.notice.is_empty(),
        };
        let display = if show { Display::Flex } else { Display::None };
        if node.display != display {
            node.display = display;
        }
    }
    for mut portrait in &mut portraits {
        if portrait.image != visuals.portraits[lab.selected] {
            portrait.image = visuals.portraits[lab.selected].clone();
        }
    }
    for (action, interaction, mut background, mut border) in &mut buttons {
        let active = match *action {
            UiAction::Mode(mode) => mode == lab.mode,
            UiAction::SelectTool(tool) => tool == lab.tool,
            UiAction::SelectActor(id) => id == lab.selected,
            UiAction::Grid => lab.grid,
            _ => false,
        };
        let disabled = matches!(action, UiAction::Confirm)
            && (lab.pending.is_none() || preview.as_ref().is_some_and(|p| !p.ok));
        let bg = if disabled {
            INK
        } else if *interaction == Interaction::Pressed {
            Color::srgb(0.23, 0.30, 0.25)
        } else if *interaction == Interaction::Hovered {
            Color::srgb(0.15, 0.23, 0.20)
        } else if active {
            Color::srgb(0.12, 0.20, 0.16)
        } else {
            PANEL
        };
        if background.0 != bg {
            background.0 = bg;
        }
        let edge = if active {
            GOLD
        } else if *interaction == Interaction::Hovered {
            MUTED
        } else {
            LINE
        };
        *border = BorderColor::all(edge);
    }
}
