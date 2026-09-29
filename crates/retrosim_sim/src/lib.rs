//! Authoritative, deterministic rules for the phase-one combat comparison.
//! Both pacing modes consume this state; clocks and presentation live outside it.

use std::collections::{HashMap, HashSet, VecDeque};

pub type Position = (i32, i32);
const STEPS: [Position; 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Team {
    Party,
    Enemy,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Player,
    Scout,
    Warden,
    Raider,
    Archer,
    Brute,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    Active,
    Won,
    Retreated,
    Defeated,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Order {
    Protect,
    Focus,
    Regroup,
    Withdraw,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InteractTarget {
    Objective,
    Resource,
    Exit,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Move { x: i32, y: i32 },
    Strike(usize),
    Bolt(usize),
    Guard,
    Heal,
    Wait,
    Interact(Option<InteractTarget>),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IntentKind {
    Attack,
    Bolt,
    Move,
    Guard,
    Wait,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EffectKind {
    Damage,
    Heal,
    Move,
    Bolt,
    Guard,
    Interact,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Intent {
    pub kind: IntentKind,
    pub target_id: Option<usize>,
    pub x: i32,
    pub y: i32,
    pub range: i32,
    pub damage: i32,
    pub text: String,
}
impl Intent {
    fn waiting(text: &str) -> Self {
        Self {
            kind: IntentKind::Wait,
            target_id: None,
            x: 0,
            y: 0,
            range: 0,
            damage: 0,
            text: text.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Actor {
    pub id: usize,
    pub team: Team,
    pub role: Role,
    pub label: &'static str,
    pub x: i32,
    pub y: i32,
    pub hp: i32,
    pub max_hp: i32,
    pub energy: i32,
    pub max_energy: i32,
    pub bolt_cooldown: u32,
    pub intent: Intent,
    pub activity: String,
    pub guard: i32,
}
impl Actor {
    pub fn position(&self) -> Position {
        (self.x, self.y)
    }
    pub fn alive(&self) -> bool {
        self.hp > 0
    }
    fn new(id: usize, role: Role, pos: Position, hp: i32) -> Self {
        let (team, label) = match role {
            Role::Player => (Team::Party, "You"),
            Role::Scout => (Team::Party, "Scout"),
            Role::Warden => (Team::Party, "Warden"),
            Role::Raider => (Team::Enemy, "Raider"),
            Role::Archer => (Team::Enemy, "Archer"),
            Role::Brute => (Team::Enemy, "Brute"),
        };
        let energy = if role == Role::Player { 6 } else { 0 };
        Self {
            id,
            team,
            role,
            label,
            x: pos.0,
            y: pos.1,
            hp,
            max_hp: hp,
            energy,
            max_energy: energy,
            bolt_cooldown: 0,
            intent: Intent::waiting("Holding position"),
            activity: "Ready".into(),
            guard: 0,
        }
    }
    // Preserve the prototype's deterministic lexical tie breaks across the Rust port.
    fn key(&self) -> &'static str {
        match self.role {
            Role::Player => "player",
            Role::Scout => "scout",
            Role::Warden => "warden",
            Role::Raider => "raider",
            Role::Archer => "archer",
            Role::Brute => "brute",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Object {
    pub x: i32,
    pub y: i32,
    pub collected: bool,
}
impl Object {
    pub fn position(&self) -> Position {
        (self.x, self.y)
    }
    fn new(x: i32, y: i32) -> Self {
        Self {
            x,
            y,
            collected: false,
        }
    }
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct LogEntry {
    pub beat: u32,
    pub text: String,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Effect {
    pub kind: EffectKind,
    pub actor_id: usize,
    pub target_id: Option<usize>,
    pub x: i32,
    pub y: i32,
    pub amount: i32,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Preview {
    pub ok: bool,
    pub title: String,
    pub detail: String,
    pub cost: String,
    pub range: i32,
    pub damage: i32,
}
impl Preview {
    fn valid(title: &str, detail: impl Into<String>, cost: &str, range: i32) -> Self {
        Self {
            ok: true,
            title: title.into(),
            detail: detail.into(),
            cost: cost.into(),
            range,
            damage: 0,
        }
    }
    fn invalid(title: &str, reason: impl Into<String>, cost: &str, range: i32) -> Self {
        Self {
            ok: false,
            ..Self::valid(title, reason, cost, range)
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct State {
    pub seed: u32,
    pub beat: u32,
    pub status: Status,
    pub width: i32,
    pub height: i32,
    pub tiles: Vec<Vec<char>>,
    pub actors: Vec<Actor>,
    pub objective: Object,
    pub resource: Object,
    pub exit: Object,
    pub order: Order,
    pub focus_id: Option<usize>,
    pub potions: u32,
    pub herbs: u32,
    pub discovery: bool,
    pub log: Vec<LogEntry>,
    pub effects: Vec<Effect>,
}

pub fn distance(a: Position, b: Position) -> i32 {
    (a.0 - b.0).abs() + (a.1 - b.1).abs()
}

impl State {
    pub fn new(seed: u32) -> Self {
        let mut state = Self {
            seed,
            beat: 0,
            status: Status::Active,
            width: 22,
            height: 12,
            tiles: terrain(),
            actors: vec![
                Actor::new(0, Role::Player, (2, 9), 24),
                Actor::new(1, Role::Scout, (1, 8), 16),
                Actor::new(2, Role::Warden, (3, 8), 22),
                Actor::new(3, Role::Raider, (12, 7), 12),
                Actor::new(4, Role::Archer, (17, 5), 10),
                Actor::new(5, Role::Brute, (18, 7), 18),
            ],
            objective: Object::new(18, 3),
            resource: Object::new(5, 3),
            exit: Object::new(1, 9),
            order: Order::Protect,
            focus_id: None,
            potions: 2,
            herbs: 0,
            discovery: false,
            log: vec![],
            effects: vec![],
        };
        match seed % 3 {
            2 => {
                state.actors[3].y = 6;
                state.actors[5].x = 19;
            }
            0 => {
                state.actors[3].x = 13;
                state.actors[4].x = 18;
            }
            _ => {}
        }
        state.record("Reach the marker, investigate it, then return to the west exit.");
        state.plan_intents();
        state
    }

    /// IDs are stable indices; actors remain in the roster after they fall.
    pub fn actor(&self, id: usize) -> &Actor {
        &self.actors[id]
    }
    pub fn is_walkable(&self, x: i32, y: i32) -> bool {
        x >= 0
            && y >= 0
            && x < self.width
            && y < self.height
            && self
                .tiles
                .get(y as usize)
                .and_then(|row| row.get(x as usize))
                .is_some_and(|c| matches!(c, '.' | ':' | '='))
    }
    fn occupied(&self, pos: Position) -> bool {
        self.actors
            .iter()
            .any(|actor| actor.alive() && actor.position() == pos)
    }

    /// Corner-inclusive ray: attacks cannot cut diagonally through blocked cells.
    pub fn has_line_of_sight(&self, from: Position, to: Position) -> bool {
        if !self.is_walkable(from.0, from.1) || !self.is_walkable(to.0, to.1) {
            return false;
        }
        let (mut x, mut y) = from;
        let (dx, dy) = (to.0 - x, to.1 - y);
        let (nx, ny) = (dx.abs(), dy.abs());
        let (sx, sy) = (dx.signum(), dy.signum());
        let (mut ix, mut iy) = (0, 0);
        while ix < nx || iy < ny {
            let decision = (1 + 2 * ix) * ny - (1 + 2 * iy) * nx;
            if decision == 0 {
                if !self.is_walkable(x + sx, y) || !self.is_walkable(x, y + sy) {
                    return false;
                }
                x += sx;
                y += sy;
                ix += 1;
                iy += 1;
            } else if decision < 0 {
                x += sx;
                ix += 1;
            } else {
                y += sy;
                iy += 1;
            }
            if !self.is_walkable(x, y) {
                return false;
            }
        }
        true
    }

    /// Bounded breadth-first search. The returned path includes the starting cell.
    pub fn find_path(
        &self,
        from: Position,
        to: Position,
        stop_range: i32,
        ignore_actors: bool,
    ) -> Vec<Position> {
        if !self.is_walkable(from.0, from.1) || stop_range < 0 {
            return vec![];
        }
        let mut queue = VecDeque::from([from]);
        let mut seen = HashSet::from([from]);
        let mut previous = HashMap::new();
        while let Some(current) = queue.pop_front() {
            if distance(current, to) <= stop_range {
                let mut path = vec![current];
                let mut cursor = current;
                while let Some(&cell) = previous.get(&cursor) {
                    path.push(cell);
                    cursor = cell;
                }
                path.reverse();
                return path;
            }
            for (dx, dy) in STEPS {
                let next = (current.0 + dx, current.1 + dy);
                if seen.contains(&next) || !self.is_walkable(next.0, next.1) {
                    continue;
                }
                if !ignore_actors && self.occupied(next) {
                    continue;
                }
                seen.insert(next);
                previous.insert(next, current);
                queue.push_back(next);
            }
        }
        vec![]
    }

    fn object(&self, target: InteractTarget) -> Object {
        match target {
            InteractTarget::Objective => self.objective,
            InteractTarget::Resource => self.resource,
            InteractTarget::Exit => self.exit,
        }
    }
    fn interaction(&self, requested: Option<InteractTarget>) -> Option<InteractTarget> {
        let available = |target| {
            let marker = self.object(target);
            distance(self.actor(0).position(), marker.position()) <= 1 && !marker.collected
        };
        match requested {
            Some(target) => available(target).then_some(target),
            None => [
                InteractTarget::Objective,
                InteractTarget::Resource,
                InteractTarget::Exit,
            ]
            .into_iter()
            .find(|&target| available(target)),
        }
    }

    pub fn preview(&self, action: &Action) -> Preview {
        let player = self.actor(0);
        if self.status != Status::Active || !player.alive() {
            return Preview::invalid("Encounter ended", "Reset to play again.", "No cost", 0);
        }
        match *action {
            Action::Move { x, y } => {
                if distance(player.position(), (x, y)) != 1 {
                    return Preview::invalid(
                        "Move",
                        "Choose one adjacent cardinal tile.",
                        "1 beat",
                        1,
                    );
                }
                if !self.is_walkable(x, y) {
                    return Preview::invalid("Move", "That tile is blocked.", "1 beat", 1);
                }
                if self.occupied((x, y)) {
                    return Preview::invalid(
                        "Move",
                        "A living actor occupies that tile.",
                        "1 beat",
                        1,
                    );
                }
                Preview::valid(
                    "Move",
                    "Move one tile; all other actors resolve one intent. No focus regeneration.",
                    "1 beat",
                    1,
                )
            }
            Action::Strike(id) | Action::Bolt(id) => {
                let bolt = matches!(action, Action::Bolt(_));
                let (title, cost, range, damage) = if bolt {
                    ("Bolt", "2 focus", 5, 4)
                } else {
                    ("Strike", "1 beat", 1, 5)
                };
                let Some(target) = self
                    .actors
                    .get(id)
                    .filter(|a| a.alive() && a.team == Team::Enemy)
                else {
                    return Preview::invalid(title, "Choose a living enemy.", cost, range);
                };
                if distance(player.position(), target.position()) > range {
                    return Preview::invalid(title, "Target is out of range.", cost, range);
                }
                if bolt && !self.has_line_of_sight(player.position(), target.position()) {
                    return Preview::invalid(
                        title,
                        "Trees, rubble, or water block line of sight.",
                        cost,
                        range,
                    );
                }
                if bolt && player.bolt_cooldown > 0 {
                    return Preview::invalid(
                        title,
                        format!(
                            "Bolt needs {} more completed beat(s).",
                            player.bolt_cooldown
                        ),
                        cost,
                        range,
                    );
                }
                if bolt && player.energy < 2 {
                    return Preview::invalid(
                        title,
                        "Bolt needs 2 focus. Wait restores 1.",
                        cost,
                        range,
                    );
                }
                let cooldown = if bolt {
                    "Uses 1 beat; 2-beat cooldown. Complete two other beats before reusing Bolt. "
                } else {
                    ""
                };
                Preview {
                    damage,
                    ..Preview::valid(
                        title,
                        format!("{damage} damage. {cooldown}Only Wait regenerates focus."),
                        cost,
                        range,
                    )
                }
            }
            Action::Guard => Preview::valid(
                "Guard",
                "Reduce every incoming hit by 3 during this beat only. No focus regeneration.",
                "1 beat",
                0,
            ),
            Action::Heal => {
                if self.potions == 0 {
                    return Preview::invalid("Heal", "No potions remaining.", "1 potion", 0);
                }
                if player.hp >= player.max_hp {
                    return Preview::invalid("Heal", "Health is already full.", "1 potion", 0);
                }
                Preview::valid(
                    "Heal",
                    format!(
                        "Uses 1 beat. Restore {} HP before other intents resolve. No focus regeneration.",
                        (player.max_hp - player.hp).min(8)
                    ),
                    "1 potion",
                    0,
                )
            }
            Action::Wait => Preview::valid(
                "Wait",
                format!(
                    "{}; all other actors resolve one intent.",
                    if player.energy < player.max_energy {
                        "Restore 1 focus (up to 6)"
                    } else {
                        "Focus is already full"
                    }
                ),
                "1 beat",
                0,
            ),
            Action::Interact(requested) => {
                let Some(kind) = self.interaction(requested) else {
                    return Preview::invalid(
                        "Interact",
                        if requested.is_some() {
                            "That object is out of reach, already collected, or unavailable."
                        } else {
                            "Stand on or beside an uncollected marker, herb, or west exit."
                        },
                        "1 beat",
                        1,
                    );
                };
                let (title, detail) = match kind {
                    InteractTarget::Objective => (
                        "Investigate marker",
                        "Acquire the discovery, then return to the west exit.",
                    ),
                    InteractTarget::Resource => ("Gather herb", "Gain 1 herb and 1 potion."),
                    InteractTarget::Exit => (
                        "Leave clearing",
                        if self.discovery {
                            "Extract with the discovery and complete this encounter."
                        } else {
                            "Retreat safely. The discovery stays behind."
                        },
                    ),
                };
                Preview::valid(
                    title,
                    format!("{detail} No focus regeneration."),
                    "1 beat",
                    1,
                )
            }
        }
    }

    pub fn advance(&mut self, action: Action) -> Result<(), String> {
        let check = self.preview(&action);
        if !check.ok {
            return Err(check.detail);
        }
        self.beat += 1;
        self.effects.clear();
        for actor in &mut self.actors {
            actor.guard = 0;
            actor.bolt_cooldown = actor.bolt_cooldown.saturating_sub(1);
        }
        self.actors[0].activity = check.title;
        match action {
            Action::Move { x, y } => self.move_actor(0, (x, y)),
            Action::Strike(target) => self.hit(0, target, 5, false),
            Action::Bolt(target) => {
                self.actors[0].energy -= 2;
                self.actors[0].bolt_cooldown = 2;
                self.hit(0, target, 4, true);
            }
            Action::Guard => {
                self.actors[0].guard = 3;
                self.effect(EffectKind::Guard, 0, None, self.actor(0).position(), 0);
                self.record("You guard: incoming hits reduced by 3 this beat.");
            }
            Action::Heal => {
                let amount = (self.actor(0).max_hp - self.actor(0).hp).min(8);
                self.potions -= 1;
                self.actors[0].hp += amount;
                self.effect(EffectKind::Heal, 0, None, self.actor(0).position(), amount);
                self.record(format!("You restore {amount} HP."));
            }
            Action::Wait => {
                let amount = (self.actor(0).max_energy - self.actor(0).energy).min(1);
                self.actors[0].energy += amount;
                self.record(if amount > 0 {
                    "You wait and recover 1 focus."
                } else {
                    "You wait. Focus is already full."
                });
            }
            Action::Interact(requested) => {
                let kind = self.interaction(requested).expect("validated interaction");
                self.effect(
                    EffectKind::Interact,
                    0,
                    None,
                    self.object(kind).position(),
                    0,
                );
                match kind {
                    InteractTarget::Objective => {
                        self.objective.collected = true;
                        self.discovery = true;
                        self.record("The marker yields a discovery. Return to the west exit.");
                    }
                    InteractTarget::Resource => {
                        self.resource.collected = true;
                        self.herbs += 1;
                        self.potions += 1;
                        self.record("Gathered 1 herb and prepared 1 potion.");
                    }
                    InteractTarget::Exit => {
                        self.status = if self.discovery {
                            Status::Won
                        } else {
                            Status::Retreated
                        };
                        self.record(if self.discovery {
                            "Discovery extracted. Encounter complete."
                        } else {
                            "Party withdrew safely."
                        });
                    }
                }
            }
        }
        if self.status == Status::Active {
            for id in 1..self.actors.len() {
                if self.actor(id).alive() {
                    self.execute_intent(id);
                }
                if !self.actor(0).alive() {
                    self.status = Status::Defeated;
                    self.record("You are down. Reset to try again.");
                    break;
                }
            }
        }
        for actor in &mut self.actors {
            actor.guard = 0;
        }
        self.plan_intents();
        Ok(())
    }

    pub fn set_order(&mut self, order: Order, target_id: Option<usize>) -> Result<(), String> {
        if self.status != Status::Active {
            return Err("Encounter ended. Reset to play again.".into());
        }
        let target = target_id.and_then(|id| self.actors.get(id));
        if order == Order::Focus && !target.is_some_and(|a| a.team == Team::Enemy && a.alive()) {
            return Err("Focus needs a living enemy target.".into());
        }
        let name = match order {
            Order::Protect => "protect",
            Order::Focus => "focus",
            Order::Regroup => "regroup",
            Order::Withdraw => "withdraw",
        };
        let suffix = if order == Order::Focus {
            format!(" {}", target.expect("validated focus").label)
        } else {
            String::new()
        };
        self.order = order;
        self.focus_id = if order == Order::Focus {
            target_id
        } else {
            None
        };
        self.record(format!("Companion order: {name}{suffix}."));
        self.plan_intents();
        Ok(())
    }

    /// A deterministic review controller. Calling it never changes the simulation.
    /// It gathers, fights, investigates, then extracts; companion orders remain explicit.
    pub fn suggest_action(&self) -> Action {
        if self.status != Status::Active {
            return Action::Wait;
        }
        if self.actor(0).hp <= 14 && self.potions > 0 {
            return Action::Heal;
        }
        let mut enemies: Vec<_> = self
            .actors
            .iter()
            .filter(|a| a.team == Team::Enemy && a.alive())
            .collect();
        enemies.sort_by_key(|a| (distance(self.actor(0).position(), a.position()), a.id));
        for enemy in &enemies {
            let action = Action::Strike(enemy.id);
            if self.preview(&action).ok {
                return action;
            }
        }
        for enemy in &enemies {
            let action = Action::Bolt(enemy.id);
            if self.preview(&action).ok {
                return action;
            }
        }
        let target = if !self.resource.collected {
            InteractTarget::Resource
        } else if !self.discovery {
            InteractTarget::Objective
        } else {
            InteractTarget::Exit
        };
        let marker = self.object(target);
        if distance(self.actor(0).position(), marker.position()) <= 1 {
            return Action::Interact(Some(target));
        }
        let path = self.find_path(self.actor(0).position(), marker.position(), 1, false);
        if let Some(&(x, y)) = path.get(1) {
            Action::Move { x, y }
        } else {
            Action::Wait
        }
    }

    fn record(&mut self, text: impl Into<String>) {
        self.log.insert(
            0,
            LogEntry {
                beat: self.beat,
                text: text.into(),
            },
        );
        self.log.truncate(30);
    }
    fn effect(
        &mut self,
        kind: EffectKind,
        actor_id: usize,
        target_id: Option<usize>,
        pos: Position,
        amount: i32,
    ) {
        self.effects.push(Effect {
            kind,
            actor_id,
            target_id,
            x: pos.0,
            y: pos.1,
            amount,
        });
    }
    fn move_actor(&mut self, id: usize, pos: Position) {
        self.actors[id].x = pos.0;
        self.actors[id].y = pos.1;
        self.actors[id].activity = "Moving".into();
        self.effect(EffectKind::Move, id, None, pos, 0);
    }
    fn hit(&mut self, actor_id: usize, target_id: usize, amount: i32, ranged: bool) {
        let damage = (amount - self.actor(target_id).guard).max(0);
        self.actors[target_id].hp = (self.actor(target_id).hp - damage).max(0);
        let pos = self.actor(target_id).position();
        if ranged {
            self.effect(EffectKind::Bolt, actor_id, Some(target_id), pos, 0);
        }
        self.effect(EffectKind::Damage, actor_id, Some(target_id), pos, damage);
        self.actors[actor_id].activity = format!("Hit {}", self.actor(target_id).label);
        self.record(format!(
            "{} hits {} for {}{}",
            self.actor(actor_id).label,
            self.actor(target_id).label,
            damage,
            if damage == 0 { " (guarded)." } else { "." }
        ));
        if !self.actor(target_id).alive() {
            self.actors[target_id].activity = "Down".into();
            self.actors[target_id].intent = Intent::waiting("Down");
            self.record(format!("{} is down.", self.actor(target_id).label));
        }
    }

    fn execute_intent(&mut self, id: usize) {
        let intent = self.actor(id).intent.clone();
        match intent.kind {
            IntentKind::Attack | IntentKind::Bolt => {
                let valid = intent
                    .target_id
                    .and_then(|target| self.actors.get(target))
                    .is_some_and(|target| {
                        target.alive()
                            && distance(self.actor(id).position(), target.position())
                                <= intent.range
                            && (intent.kind != IntentKind::Bolt
                                || self.has_line_of_sight(
                                    self.actor(id).position(),
                                    target.position(),
                                ))
                    });
                if valid {
                    self.hit(
                        id,
                        intent.target_id.expect("validated attack"),
                        intent.damage,
                        intent.kind == IntentKind::Bolt,
                    );
                } else {
                    self.actors[id].activity = "Attack canceled".into();
                    self.record(format!(
                        "{}'s attack canceled: target moved, is down, or sight is blocked.",
                        self.actor(id).label
                    ));
                }
            }
            IntentKind::Move => {
                if self.is_walkable(intent.x, intent.y) && !self.occupied((intent.x, intent.y)) {
                    self.move_actor(id, (intent.x, intent.y));
                } else {
                    self.actors[id].activity = "Move blocked".into();
                    self.record(format!(
                        "{}'s move canceled: tile occupied.",
                        self.actor(id).label
                    ));
                }
            }
            IntentKind::Guard => {
                self.actors[id].guard = 3;
                self.actors[id].activity = "Guarding".into();
                self.effect(EffectKind::Guard, id, None, self.actor(id).position(), 0);
            }
            IntentKind::Wait => self.actors[id].activity = intent.text,
        }
    }

    fn approach(
        &self,
        id: usize,
        target: Position,
        target_id: Option<usize>,
        stop_range: i32,
        text: &str,
    ) -> Intent {
        let path = self.find_path(self.actor(id).position(), target, stop_range, false);
        if let Some(&(x, y)) = path.get(1) {
            Intent {
                kind: IntentKind::Move,
                target_id,
                x,
                y,
                range: 0,
                damage: 0,
                text: text.into(),
            }
        } else {
            Intent::waiting("Holding position")
        }
    }
    fn attack_intent(&self, id: usize, target_id: usize) -> Intent {
        let actor = self.actor(id);
        let target = self.actor(target_id);
        let ranged = matches!(actor.role, Role::Archer | Role::Scout);
        let range = match actor.role {
            Role::Archer => 5,
            Role::Scout => 4,
            _ => 1,
        };
        let damage = match actor.role {
            Role::Brute => 5,
            Role::Warden => 4,
            _ => 3,
        };
        if distance(actor.position(), target.position()) <= range
            && (!ranged || self.has_line_of_sight(actor.position(), target.position()))
        {
            Intent {
                kind: if ranged {
                    IntentKind::Bolt
                } else {
                    IntentKind::Attack
                },
                target_id: Some(target_id),
                x: target.x,
                y: target.y,
                range,
                damage,
                text: format!(
                    "Aim at {}: {} damage if still in range{}",
                    target.label,
                    damage,
                    if ranged { " and sight" } else { "" }
                ),
            }
        } else {
            self.approach(
                id,
                target.position(),
                Some(target_id),
                1,
                &format!("Approach {}", target.label),
            )
        }
    }
    fn nearest(&self, id: usize, filter: impl Fn(&Actor) -> bool) -> Option<usize> {
        self.actors
            .iter()
            .filter(|a| a.alive() && filter(a))
            .min_by_key(|a| (distance(self.actor(id).position(), a.position()), a.key()))
            .map(|a| a.id)
    }
    fn plan_intents(&mut self) {
        let intents: Vec<_> = self
            .actors
            .iter()
            .map(|actor| {
                if !actor.alive() || self.status != Status::Active {
                    return Intent::waiting(if actor.alive() {
                        "Encounter ended"
                    } else {
                        "Down"
                    });
                }
                let id = actor.id;
                if id == 0 {
                    return Intent::waiting("Your next action");
                }
                if actor.team == Team::Enemy {
                    return match self.nearest(id, |a| a.team == Team::Party) {
                        Some(target)
                            if distance(actor.position(), self.actor(target).position()) <= 7 =>
                        {
                            self.attack_intent(id, target)
                        }
                        _ => Intent::waiting("Watching the clearing"),
                    };
                }
                if self.order == Order::Withdraw {
                    return self.approach(
                        id,
                        self.exit.position(),
                        None,
                        1,
                        "Withdraw to west exit",
                    );
                }
                if self.order == Order::Regroup {
                    return self.approach(
                        id,
                        self.actor(0).position(),
                        Some(0),
                        1,
                        "Regroup near you",
                    );
                }
                let focused = if self.order == Order::Focus {
                    self.focus_id.filter(|&target| {
                        self.actors
                            .get(target)
                            .is_some_and(|a| a.team == Team::Enemy && a.alive())
                    })
                } else {
                    None
                };
                let target = focused.or_else(|| {
                    self.nearest(id, |a| {
                        a.team == Team::Enemy
                            && distance(self.actor(0).position(), a.position()) <= 5
                    })
                });
                if let Some(target) = target {
                    self.attack_intent(id, target)
                } else if distance(actor.position(), self.actor(0).position()) > 2 {
                    self.approach(id, self.actor(0).position(), Some(0), 2, "Follow you")
                } else {
                    Intent {
                        kind: IntentKind::Guard,
                        text: "Guard nearby position: block 3 this beat".into(),
                        ..Intent::waiting("")
                    }
                }
            })
            .collect();
        for (actor, intent) in self.actors.iter_mut().zip(intents) {
            actor.intent = intent;
        }
    }
}

fn terrain() -> Vec<Vec<char>> {
    let mut rows = vec![vec!['.'; 22]; 12];
    for (y, row) in rows.iter_mut().enumerate() {
        for (x, tile) in row.iter_mut().enumerate() {
            if x == 0 || x == 21 || y == 0 || y == 11 {
                *tile = '#';
            }
        }
    }
    let mut paint = |kind, cells: &[(usize, usize)]| {
        for &(x, y) in cells {
            rows[y][x] = kind;
        }
    };
    paint(
        '#',
        &[
            (1, 1),
            (2, 1),
            (3, 1),
            (1, 2),
            (2, 2),
            (7, 1),
            (8, 1),
            (8, 2),
            (9, 2),
            (6, 4),
            (6, 5),
            (7, 5),
            (8, 5),
            (9, 5),
            (10, 5),
            (10, 6),
            (3, 5),
            (3, 6),
            (4, 6),
            (1, 7),
            (2, 7),
            (6, 10),
            (7, 10),
            (8, 10),
            (12, 9),
            (12, 10),
            (13, 10),
            (20, 8),
            (20, 9),
            (19, 10),
            (20, 10),
        ],
    );
    paint(
        '~',
        &[
            (11, 1),
            (12, 1),
            (11, 2),
            (12, 2),
            (13, 2),
            (12, 3),
            (13, 3),
            (14, 3),
        ],
    );
    paint(
        'R',
        &[
            (16, 1),
            (17, 1),
            (19, 1),
            (20, 1),
            (16, 2),
            (20, 2),
            (16, 3),
            (20, 3),
            (19, 4),
            (20, 4),
            (15, 7),
            (16, 7),
        ],
    );
    paint(
        ':',
        &[
            (1, 9),
            (2, 9),
            (3, 9),
            (4, 9),
            (4, 8),
            (5, 8),
            (6, 8),
            (7, 8),
            (8, 8),
            (8, 7),
            (9, 7),
            (10, 7),
            (11, 7),
            (11, 6),
            (12, 6),
            (13, 6),
            (14, 6),
            (14, 5),
            (15, 5),
            (16, 5),
            (17, 5),
            (17, 4),
            (4, 7),
            (5, 7),
            (5, 6),
            (5, 5),
            (5, 4),
            (5, 3),
            (6, 3),
            (7, 3),
            (8, 3),
            (9, 3),
            (9, 4),
            (10, 4),
            (11, 4),
            (12, 4),
            (13, 4),
            (14, 4),
            (15, 4),
        ],
    );
    paint(
        '=',
        &[
            (17, 2),
            (18, 2),
            (19, 2),
            (17, 3),
            (18, 3),
            (19, 3),
            (18, 4),
        ],
    );
    rows
}

#[cfg(test)]
mod tests {
    use super::*;

    fn no_overlap(state: &State) {
        let alive: Vec<_> = state.actors.iter().filter(|a| a.alive()).collect();
        let positions: HashSet<_> = alive.iter().map(|a| a.position()).collect();
        assert_eq!(positions.len(), alive.len(), "Living actors overlap");
        for actor in alive {
            assert!(
                state.is_walkable(actor.x, actor.y),
                "{} on blocked terrain",
                actor.label
            );
        }
    }
    fn duel() -> State {
        let mut state = State::new(1);
        state.actors[0].x = 9;
        state.actors[0].y = 7;
        for actor in &mut state.actors {
            if actor.id != 0 && actor.id != 3 {
                actor.hp = 0;
            }
        }
        state.actors[3].x = 11;
        state.actors[3].y = 7;
        state.set_order(Order::Protect, None).unwrap();
        state
    }
    fn adventure(seed: u32) -> State {
        let mut state = State::new(seed);
        for _ in 0..220 {
            if state.status != Status::Active {
                break;
            }
            let threat = state
                .actors
                .iter()
                .filter(|a| {
                    a.team == Team::Enemy
                        && a.alive()
                        && distance(state.actor(0).position(), a.position()) <= 6
                })
                .min_by_key(|a| (distance(state.actor(0).position(), a.position()), a.id))
                .map(|a| a.id);
            if let Some(id) = threat {
                if state.focus_id != Some(id) {
                    state.set_order(Order::Focus, Some(id)).unwrap();
                }
            }
            let action = state.suggest_action();
            assert!(
                state.preview(&action).ok,
                "{action:?} at beat {}",
                state.beat
            );
            state.advance(action).unwrap();
            no_overlap(&state);
        }
        state
    }

    #[test]
    fn invalid_actions_never_tick_or_mutate() {
        let mut state = State::new(1);
        let before = state.clone();
        for action in [
            Action::Move { x: 8, y: 9 },
            Action::Move { x: 2, y: 11 },
            Action::Bolt(5),
            Action::Bolt(99),
            Action::Strike(1),
            Action::Heal,
        ] {
            assert!(!state.preview(&action).ok);
            assert!(state.advance(action).is_err());
            assert_eq!(state, before);
        }
    }
    #[test]
    fn explicit_interaction_never_falls_back() {
        let mut state = State::new(1);
        let before = state.clone();
        for target in [InteractTarget::Objective, InteractTarget::Resource] {
            assert!(!state.preview(&Action::Interact(Some(target))).ok);
            assert!(state.advance(Action::Interact(Some(target))).is_err());
            assert_eq!(state, before);
        }
        state.actors[0].x = 5;
        state.actors[0].y = 4;
        state
            .advance(Action::Interact(Some(InteractTarget::Resource)))
            .unwrap();
        assert_eq!((state.herbs, state.potions), (1, 3));
        state.actors[0].x = 18;
        state.actors[0].y = 3;
        state
            .advance(Action::Interact(Some(InteractTarget::Objective)))
            .unwrap();
        assert!(state.discovery);
        state.exit = Object::new(18, 4);
        assert_eq!(
            state.preview(&Action::Interact(None)).title,
            "Leave clearing"
        );
        let before = state.clone();
        assert!(
            state
                .advance(Action::Interact(Some(InteractTarget::Objective)))
                .is_err()
        );
        assert_eq!(state, before);
        state.exit = Object::new(1, 9);
        state.actors[0].x = 2;
        state.actors[0].y = 9;
        state
            .advance(Action::Interact(Some(InteractTarget::Exit)))
            .unwrap();
        assert_eq!(state.status, Status::Won);
    }
    #[test]
    fn movement_bounds_terrain_and_live_occupancy() {
        let mut state = State::new(1);
        assert!(state.preview(&Action::Move { x: 3, y: 9 }).ok);
        assert!(state.preview(&Action::Move { x: 2, y: 8 }).ok);
        state.actors[1].x = 2;
        state.actors[2].y = 9;
        assert!(!state.preview(&Action::Move { x: 3, y: 9 }).ok);
        assert!(!state.preview(&Action::Move { x: 2, y: 8 }).ok);
        state.actors[0].x = 1;
        state.actors[0].y = 8;
        for (x, y) in [(0, 8), (1, 7), (-1, 8), (22, 8), (1, 12)] {
            assert!(!state.preview(&Action::Move { x, y }).ok);
        }
        state.actors[0].x = 2;
        state.actors[0].y = 9;
        state.actors[1].hp = 0;
        state.advance(Action::Move { x: 2, y: 8 }).unwrap();
        assert_eq!(state.actor(0).position(), (2, 8));
        no_overlap(&state);
    }
    #[test]
    fn line_of_sight_blocks_obstacles_and_corners() {
        let mut state = State::new(1);
        assert!(state.has_line_of_sight((9, 7), (14, 7)));
        assert!(!state.has_line_of_sight((5, 4), (7, 4)));
        assert!(!state.has_line_of_sight((5, 4), (6, 3)));
        state.actors[0].x = 5;
        state.actors[0].y = 4;
        state.actors[3].x = 7;
        state.actors[3].y = 4;
        assert!(
            state
                .preview(&Action::Bolt(3))
                .detail
                .contains("line of sight")
        );
    }
    #[test]
    fn bolt_preview_damage_focus_and_two_intervening_beats() {
        let mut state = duel();
        state.actors[3].intent = Intent::waiting("Holding");
        let preview = state.preview(&Action::Bolt(3));
        assert_eq!((preview.damage, preview.range), (4, 5));
        state.advance(Action::Bolt(3)).unwrap();
        assert_eq!(
            (
                state.actor(3).hp,
                state.actor(0).energy,
                state.actor(0).bolt_cooldown
            ),
            (8, 4, 2)
        );
        assert!(!state.preview(&Action::Bolt(3)).ok);
        state.advance(Action::Guard).unwrap();
        assert_eq!(
            (state.actor(0).bolt_cooldown, state.actor(0).energy),
            (1, 4)
        );
        assert!(!state.preview(&Action::Bolt(3)).ok);
        state.advance(Action::Wait).unwrap();
        assert_eq!(
            (state.actor(0).bolt_cooldown, state.actor(0).energy),
            (0, 5)
        );
        assert!(state.preview(&Action::Bolt(3)).ok);
    }
    #[test]
    fn strike_heal_and_focus_match_preview() {
        let mut state = duel();
        state.actors[3].x = 10;
        state.actors[3].intent = Intent::waiting("Holding");
        state.actors[3].guard = 3; // Prior-beat guard expires, so it must not be subtracted twice.
        let expected = state.preview(&Action::Strike(3)).damage;
        state.advance(Action::Strike(3)).unwrap();
        assert_eq!(12 - state.actor(3).hp, expected);
        assert_eq!(state.actor(0).energy, 6);
        state.actors[0].hp = 14;
        state.actors[3].hp = 0;
        state.advance(Action::Heal).unwrap();
        assert_eq!((state.actor(0).hp, state.potions), (22, 1));
        assert_eq!(
            state
                .effects
                .iter()
                .find(|e| e.kind == EffectKind::Heal)
                .unwrap()
                .amount,
            8
        );
    }
    #[test]
    fn telegraphed_attacks_cancel_and_guard_expires_after_one_beat() {
        let mut state = duel();
        state.actors[3].x = 10;
        state.set_order(Order::Protect, None).unwrap();
        assert_eq!(state.actor(3).intent.kind, IntentKind::Attack);
        state.advance(Action::Move { x: 8, y: 7 }).unwrap();
        assert_eq!(state.actor(0).hp, 24);
        assert!(state.log[0].text.contains("canceled"));
        state.actors[0].x = 9;
        state.actors[3].x = 10;
        state.set_order(Order::Protect, None).unwrap();
        state.advance(Action::Guard).unwrap();
        assert_eq!((state.actor(0).hp, state.actor(0).guard), (24, 0));
        state.advance(Action::Wait).unwrap();
        assert_eq!(state.actor(0).hp, 21);
    }
    #[test]
    fn free_orders_and_autonomous_companions() {
        let mut state = State::new(1);
        state.set_order(Order::Focus, Some(3)).unwrap();
        assert_eq!(state.beat, 0);
        let start = state.actor(1).position();
        for _ in 0..14 {
            if state.status != Status::Active {
                break;
            }
            state.advance(Action::Wait).unwrap();
            no_overlap(&state);
        }
        assert_ne!(state.actor(1).position(), start);
        assert!(state.actor(3).hp < 12);
        state.set_order(Order::Regroup, None).unwrap();
        state.set_order(Order::Withdraw, None).unwrap();
        assert!(state.set_order(Order::Focus, Some(0)).is_err());
    }
    #[test]
    fn actual_clearing_is_reachable_and_every_seeded_layout_is_winnable() {
        for seed in [1, 2, 3] {
            let initial = State::new(seed);
            assert_eq!(initial.tiles.len(), 12);
            assert!(initial.tiles.iter().all(|row| row.len() == 22));
            assert!(
                !initial
                    .find_path(
                        initial.actor(0).position(),
                        initial.resource.position(),
                        1,
                        true
                    )
                    .is_empty()
            );
            assert!(
                !initial
                    .find_path(
                        initial.resource.position(),
                        initial.objective.position(),
                        1,
                        true
                    )
                    .is_empty()
            );
            let state = adventure(seed);
            assert_eq!(
                state.status,
                Status::Won,
                "Seed {seed} at beat {}",
                state.beat
            );
            assert!(state.discovery);
            assert_eq!(state.herbs, 1);
        }
    }
    #[test]
    fn deterministic_replay_matches_complete_state() {
        let mut source = State::new(1);
        let mut replay = source.clone();
        for _ in 0..220 {
            if source.status != Status::Active {
                break;
            }
            let threat = source
                .actors
                .iter()
                .filter(|a| {
                    a.team == Team::Enemy
                        && a.alive()
                        && distance(source.actor(0).position(), a.position()) <= 6
                })
                .min_by_key(|a| (distance(source.actor(0).position(), a.position()), a.id))
                .map(|a| a.id);
            if let Some(id) = threat {
                if source.focus_id != Some(id) {
                    source.set_order(Order::Focus, Some(id)).unwrap();
                    replay.set_order(Order::Focus, Some(id)).unwrap();
                }
            }
            let action = source.suggest_action();
            source.advance(action).unwrap();
            replay.advance(action).unwrap();
            assert_eq!(source, replay);
            no_overlap(&source);
        }
        assert_eq!(source.status, Status::Won);
    }
    #[test]
    fn retreat_win_and_defeat_freeze_state() {
        let mut retreat = State::new(1);
        retreat.advance(Action::Interact(None)).unwrap();
        assert_eq!(retreat.status, Status::Retreated);
        let won = adventure(1);
        let mut defeated = duel();
        defeated.actors[3].x = 10;
        defeated.actors[0].hp = 1;
        defeated.set_order(Order::Protect, None).unwrap();
        defeated.advance(Action::Wait).unwrap();
        assert_eq!(defeated.status, Status::Defeated);
        for mut state in [retreat, won, defeated] {
            let before = state.clone();
            assert!(state.advance(Action::Wait).is_err());
            assert!(state.set_order(Order::Protect, None).is_err());
            assert_eq!(state, before);
        }
    }
    #[test]
    fn long_autonomous_battle_has_bounded_state_and_no_overlaps() {
        let mut state = State::new(29);
        state.set_order(Order::Focus, Some(5)).unwrap();
        for _ in 0..150 {
            if state.status != Status::Active {
                break;
            }
            state
                .advance(if state.actor(0).hp < 16 && state.potions > 0 {
                    Action::Heal
                } else {
                    Action::Wait
                })
                .unwrap();
            no_overlap(&state);
            assert!(state.log.len() <= 30);
            for actor in &state.actors {
                assert!((0..=actor.max_hp).contains(&actor.hp));
            }
        }
    }
}
