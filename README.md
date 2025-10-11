Generate an ical/ics file with birthdays. Reads from the file `birthdays.txt` which has the following format:

```
  1937-02-21 Harald V         # King of Norway
  178?-03-18 Miloš Obrenović  # Known date, but not year; will show up without age
  002?-??-?? Pliny the Elder  # Not sure about birthday; will be ignored

# Ignoring blank lines and comments are optional:

  1941-01-05 Hayao Miyazaki
```

## TODO

- generate more birthday events, past and present
- add recurring events for people without known age
