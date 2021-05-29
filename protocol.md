# Requests

```
SET %<u8> # set speed percentage
SET v<u16> # set voltage in millivolts
SET a # let the device decide

ADJ v<i16> # adjust the voltage in millivolts
ADJ %<i8> # adjust the speed percentage

GET volt # get current voltage in millivolts
get cfg # get if manual or auto
GET temp # get temperature in celcius
GET % # get speed percentage
```

# Encoding

ASCII.

# Connection

Any lossless connection like TCP or Serial Over Bluetooth.

# Messages

`<RequestType> <value>`

Messages are terminated by a `\0` (null byte).

# Responses

There're two response types:

-	Ok: status code 0.
-	Err: status code 1.

Response format is SON.

## Sample Response

```JSON
{"status": 0, "msg": "fan set to 100%"}
```
