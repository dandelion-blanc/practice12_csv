//! # Practice12 csv
//! csvファイルの入出力練習
//!

extern crate csv;
extern crate serde;
#[macro_use]
extern crate serde_derive;


use std::env;
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
struct Record<'a>
{
	city: &'a str,
	state: &'a str,
	population: Option<u64>,
	latitude: f64,
	longitude: f64,
}

fn out_normal(filename :&str) -> Result<(), Box<dyn Error>>
{
	let mut wtr = csv::WriterBuilder::new()
							.quote_style(csv::QuoteStyle::NonNumeric)
							.from_path(filename)?;

	wtr.write_record(&["City", "State", "Population", "Latitude", "Longitude"])?;
	wtr.write_record(&["Davidsons Landing", "AK", "", "65.2419444", "-165.2716667"])?;
	wtr.write_record(&["Kenai", "AK", "7610", "60.5544444", "-151.2583333"])?;
	wtr.write_record(&["Oakman", "AL", "", "33.7133333", "-87.3886111"])?;

	wtr.flush()?;
	Ok(())
}

fn out_serde(filename :&str) -> Result<(), Box<dyn Error>>
{
	let mut wtr = csv::Writer::from_path(filename)?;

	let city = vec!["Davidsons Landing", "Kenai", "Oakman"];
	let state = vec!["AK", "AK", "AL"];
	let population = vec![None, Some(7610), None];
	let latitude = vec![65.2419444, 60.5544444, 33.7133333];
	let longitude = vec![-165.2716667, -151.2583333, -87.3886111];

	for ((((city, state), population), latitude), longitude) in city.iter().zip(state).zip(population).zip(latitude).zip(longitude)
	{
		let record = Record
							{
								city: city,
								state: state,
								population: population,
								latitude: latitude,
								longitude: longitude,
							};
		wtr.serialize(record)?;
	}

    wtr.flush()?;
    Ok(())
}

fn main()
{
	out_normal(&"out_normal.csv");
	out_serde(&"out_serde.csv");
}
