# RAD Object Notation

## Structure
  RAD object notation could have the following fields:

### name
  Name of the RAD request.

### notBefore
  ???

### retrive
  _Retrieve information_. Retrieve in itself could have the following fields:

#### sources
  _Sources information_. Sources could have:

##### script
  _Sources information_. Data normalization happens here. Array of objects that contains (only ???) functions. Avaible functions are:

  ###### Object.getResult Any ???
  ###### Object.toJSON -> String  | Object.toString -> String
  ###### Object.toXML -> String   | Object.toString -> String
  ###### String.asString -> String
  ###### String.toXML -> Object
  ###### String.toXML -> Object
  ###### String.toLowerCase -> String
  ###### String.toUpperCase -> String
  ###### String.hash -> String
  ###### String.toInt -> Int
  ###### String.asInt -> Int
  ###### String.asFloat -> Float



#### aggregator
  _Aggregator information_.

### attest
  _Attest_information_.

### deliver
  _deliver information_.

## Example
```
{
  name: "WeatherInLondon",
  notBefore: 12345678,
  retrieve: {
    sources: [
      {
        type: "curl",
        name: "OpenWeatherMap",
        url: "https://api.openweathermap.org/data/2.5/weather?q=London,uk",
        script: [
          { f: "getResult" },
          { f: "parseFromJSON" },
          { f: "get", params: { key: "weather" } },
          { f: "get", params: { key: 0 } },
          { f: "get", params: { key: "main"} },
          { f: "toLowerCase" }
        ]
      }
    ],
    aggregator: "equality",
  },
  attest: {
    consensus: "equality"
  },
  deliver: null
}
```