/*  
 * Copyright 2026 SoTeen Studio
 *  
 * Licensed under the Apache License, Version 2.0 (the "License");  
 * you may not use this file except in compliance with the License.  
 * You may obtain a copy of the License at  
 *  
 *     http://www.apache.org/licenses/LICENSE-2.0  
 */

pub mod analyze_usage;
pub mod eliminate_dead_loops;
pub mod eliminate_dead_stores;
pub mod eliminate_redundant_loads;
pub mod fold_constants;
pub mod fold_conversions;
pub mod is_pure_loop;
pub mod jump_threading;
pub mod optimize_bytecode;
pub mod strength_reduction;
