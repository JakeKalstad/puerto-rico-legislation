# PR LEGISLATION

Making the legislation process in puerto rico more transparent 

## Scrape legislation past and present

<http://www.tucamarapr.org/dnncamara/web/ActividadLegislativa/tramitelegislativo.aspx?measureid=XXXX>

contents to be retrieved and stored as JSON files to be processed later
save as documents/{measure_name}.es.json

    measure {
        Measure Name :: string
        Date Filed :: date
        Authors :: string[]
        Heading :: string
        History :: History[]
    }

    History {
        Date :: date
        Description :: string 
        Document :: string (url)
    }

## download all associated documents

save contents to folders documents/{measure_name}/{history_date}.{history_description}.pdf

## Scrape vote date ??? how to find vote IDs? aspx sucks

<http://www.tucamarapr.org/dnncamara/web/ActividadLegislativa/votaciones.aspx?measureid=XXX&voteid=???>

## translation

translate documents/{measure_name}.es.json into documents/{measure_name}.en.json

## create table html

index page for es/en
filter by measure id/heading substring/authors

## generate html for each measure

convert json files into static html files for es/en
