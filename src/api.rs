use chrono::{DateTime, Local};
use reqwest::{blocking::Client as reqwestClient, header as reqwestHeader};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use url::{ParseError, Url};

#[derive(Debug)]
pub struct Availability {
    pub room: String,
    pub time_available: HashMap<String, bool>,
}

pub struct Client {
    url: String,
    user: String,
    password: String,

    client: reqwestClient,
}

impl Client {
    fn get_availability(
        &self,
        guid: &str,
        date: DateTime<Local>,
    ) -> Result<GetAvailabilityResponse, Box<dyn std::error::Error>> {
        let mut query_url = self.build_client_url("/en/bookings/GetAvailabilityAtWithUser")?;
        query_url
            .query_pairs_mut()
            .append_pair("guid", guid)
            .append_pair("startTime", format!("{}", date.format("%Y-%m-%d")).as_str())
            .append_pair("interval", "30");

        let resp: GetAvailabilityResponse = self
            .client
            .get(query_url)
            .basic_auth(self.user.as_str(), Some(self.password.as_str()))
            .header(reqwestHeader::ACCEPT, "application/json")
            .header(reqwestHeader::CONTENT_TYPE, "application/json")
            .send()?
            .json()?;

        Ok(resp)
    }

    pub fn new(url: String, user: String, password: String) -> Self {
        Self {
            url,
            user,
            password,
            client: reqwestClient::new(),
        }
    }

    pub fn rooms_availability(
        &self,
        guid: &str,
        date: DateTime<Local>,
    ) -> Result<Availability, Box<dyn std::error::Error>> {
        let room_availability_response = self.get_availability(guid, date)?;
        let mut room_availability_room = Availability {
            room: room_availability_response.resource.name,
            time_available: HashMap::new(),
        };

        for slot in room_availability_response.available_slots.iter() {
            room_availability_room
                .time_available
                .insert(slot.time.to_owned(), !slot.booked.to_owned());
        }

        Ok(room_availability_room)
    }
    fn build_client_url(&self, path: &str) -> Result<Url, ParseError> {
        let base = Url::parse(self.url.as_str()).expect("hardcoded URL is known to be valid");
        let joined = base.join(path)?;

        Ok(joined)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct GetAvailabilityResponse {
    #[serde(rename = "Resource")]
    pub resource: Resource,
    #[serde(rename = "AvailableSlots")]
    pub available_slots: Vec<AvailableSlot>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct Resource {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Id")]
    pub id: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
struct AvailableSlot {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Time")]
    pub time: String,
    #[serde(rename = "Available")]
    pub available: bool,
    #[serde(rename = "AllowMultipleBookings")]
    pub allow_multiple_bookings: bool,
    #[serde(rename = "Capacity")]
    pub capacity: i64,
    #[serde(rename = "BookedCount")]
    pub booked_count: i64,
    #[serde(rename = "Booked")]
    pub booked: bool,
}
