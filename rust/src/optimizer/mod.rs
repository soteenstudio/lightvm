/*
 * Copyright 2026 SoTeen Studio
 *
 * Licensed under the Apache License, Version 2.0 (the "License")
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 */

pub(crate) mod analyze_usage;
pub(crate) mod eliminate_dead_loops;
pub(crate) mod eliminate_dead_stores;
pub(crate) mod eliminate_redundant_loads;
pub(crate) mod fold_constants;
pub(crate) mod fold_conversions;
pub(crate) mod is_pure_loop;
pub(crate) mod jump_threading;
pub(crate) mod optimize_bytecode;
pub(crate) mod strength_reduction;
