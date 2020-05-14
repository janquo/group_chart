use crate::Album;
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

impl Album {
    pub fn to_html_card(&self) -> String {
        format!(
            include_str!("../data/html_card"),
            match &self.image {
                Some(x) => &x[..],
                None => "blank.png",
            },
            self.artist,
            self.title,
            self.playcount,
            match &self.score {
                Some(x) => (*x.numer() as f64) / (*x.denom() as f64),
                None => 0.0,
            },
            self.no_contributors,
            self.best_contributor.0,
            self.best_contributor.1,
        )
    }

    pub fn to_html_card_with_chart(&self, number: usize) -> String {
        format!(
            include_str!("../data/html_card_chart"),
            match &self.image {
                Some(x) => &x[..],
                None => "blank.png",
            },
            format!("carousel{}controls", number),
            format!("chart{}", number),
            self.artist,
            self.title,
            self.playcount,
            match &self.score {
                Some(x) => (*x.numer() as f64) / (*x.denom() as f64),
                None => 0.0,
            },
            self.no_contributors,
            self.best_contributor.0,
            self.best_contributor.1,
            format!("#carousel{}controls", number),
            format!("#carousel{}controls", number)
        )
    }
}

pub fn albums_to_html(albums: &[&Album], charts: bool) -> String {
    let mut doc = String::from(include_str!("../data/html_header"));
    if charts {
        for (i, album) in albums.iter().enumerate() {
            doc.push_str(&album.to_html_card_with_chart(i));
        }
    } else {
        for album in albums {
            doc.push_str(&album.to_html_card());
        }
    }
    doc.push_str(include_str!("../data/html_footer"));
    doc
}

pub fn charts_js(data: Vec<Vec<(usize, usize, usize)>>) -> String {
    let n = data.len();
    let mut data_js = String::from("");
    for mut row in data.into_iter() {
        row.sort_by_key(|x| x.0);
        let mut i = 1;
        while i < row.len() {
            if row[i].0 - 1 > row[i - 1].0 {
                row.insert(i, (row[i - 1].0 + 1, 50, 0));
            }
            i += 1;
        }
        data_js.push_str(&format!(
            "chartData.push({:?})
             chartData.push({:?})\n",
            row.iter().map(|v| v.2).collect::<Vec<usize>>(),
            row.iter().map(|v| v.1).collect::<Vec<usize>>()
        ));
    }
    format!(include_str!("../data/charts_script"), n, data_js)
}

pub fn save_index_html(s: &str, path: &Path) -> io::Result<()> {
    let mut file = File::create(path.join("index.html"))?;
    file.write_all(s.as_bytes())?;
    Ok(())
}

pub fn save_charts_script(s: &str, path: &Path) -> io::Result<()> {
    let mut file = File::create(path.join("history_charts.js"))?;
    file.write_all(s.as_bytes())?;
    Ok(())
}
