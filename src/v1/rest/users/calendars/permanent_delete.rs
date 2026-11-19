//! Permanently delete a Microsoft Graph calendar
//! (`POST /me/calendars/{id}/permanentDelete`).
//!
//! A plain delete moves the calendar to Deleted Items, where it stays until
//! that folder is emptied; this one removes it to the Recoverable Items
//! folder at once.
//!
//! <https://learn.microsoft.com/en-us/graph/api/calendar-permanentdelete>

use alloc::format;

use io_http::rfc6750::bearer::HttpAuthBearer;
use log::{debug, trace};
use url::Url;

use crate::{
    coroutine::*,
    msgraph_try,
    v1::send::{
        MSGRAPH_API_BASE, MsgraphNoResponse, MsgraphSend, MsgraphSendError, MsgraphSendOutput,
        path_segment, user_path,
    },
};

/// Permanently deletes a Microsoft Graph calendar.
pub struct MsgraphCalendarPermanentDelete {
    send: MsgraphSend<MsgraphNoResponse>,
}

impl MsgraphCalendarPermanentDelete {
    /// Permanently deletes the calendar `id`.
    pub fn new(auth: &HttpAuthBearer, user_id: &str, id: &str) -> Result<Self, MsgraphSendError> {
        debug!("prepare microsoft graph calendar permanent deletion");
        trace!("id: {id:?}");

        let user = user_path(user_id)?;
        let url = Url::parse(MSGRAPH_API_BASE)?.join(&format!(
            "{user}/calendars/{}/permanentDelete",
            path_segment(id)?
        ))?;
        let body = b"{}".to_vec();
        let send = MsgraphSend::with_method(auth, "POST", url, Some("application/json"), body);

        Ok(Self { send })
    }
}

impl MsgraphCoroutine for MsgraphCalendarPermanentDelete {
    type Yield = MsgraphYield;
    type Return = Result<MsgraphSendOutput<MsgraphNoResponse>, MsgraphSendError>;

    fn resume(&mut self, arg: Option<&[u8]>) -> MsgraphCoroutineState<Self::Yield, Self::Return> {
        let out = msgraph_try!(&mut self.send, arg);
        debug!("calendar permanently deleted");
        trace!("out: {out:?}");
        MsgraphCoroutineState::Complete(Ok(out))
    }
}
