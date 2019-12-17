/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/. */

use crate::script_runtime::{CommonScriptMsg, ScriptChan, ScriptThreadEventCategory};
use crate::task::{TaskCanceller, TaskOnce};
use crate::task_source::{TaskSource, TaskSourceName};
use msg::constellation_msg::PipelineId;

#[derive(JSTraceable)]
pub struct DatabaseAccessTaskSource(pub Box<dyn ScriptChan + Send + 'static>, pub PipelineId);

impl Clone for DatabaseAccessTaskSource {
    fn clone(&self) -> DatabaseAccessTaskSource {
        DatabaseAccessTaskSource(self.0.clone(), self.1.clone())
    }
}

impl TaskSource for DatabaseAccessTaskSource {
    const NAME: TaskSourceName = TaskSourceName::DatabaseAccess;

    fn queue_with_canceller<T>(&self, task: T, canceller: &TaskCanceller) -> Result<(), ()>
    where
        T: TaskOnce + 'static,
    {
        self.0.send(CommonScriptMsg::Task(
            ScriptThreadEventCategory::DatabaseAccessEvent,
            Box::new(canceller.wrap_task(task)),
            Some(self.1),
            DatabaseAccessTaskSource::NAME,
        ))
    }
}
