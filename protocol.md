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
GET all # get all possible readings
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

The response format is `CODE:MSG`, where
-	CODE: the status code
-	MSG: the message string

## Sample Response

`0:voltage set to 3.3v`
