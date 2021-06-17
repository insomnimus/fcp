# Fan Control Protocol (FCP)

A tiny protocol to transmit fan control messages.

Written for systems where resources are not taken as granted.

For example there's absolutely no stdlib dependency (no_std).
This crate also doesn't assume any sort of heap allocator; all operations use the stack and very sparingly .

FCP does not use floating point numbers.
The fine tuning is done by assuming values are, for example in millivolts instead of volts.
The reasoning behind this is, not all microcontrollers come with a floating point unit.

# Stability and Versioning

Any major release is not backwards compatible.

Any new minor release, up to 1.0 may or may not break backwards compatibility.
Any new minor release after 1.0 will not break backwards compatibility unless the release is security related or critical.

Any new patch release is always backwards compatible.

# Requests

### SET

SET requests are made to set a variable such as the operating speed percentage.

The syntax for SET requests is:

`SET <CHAR>[VALUE]`

where

-	CHAR: a one byte character, denoting the target to set. Letters must be lowercase.
-	VALUE: any ASCII encoded string without any space characters.

```
SET %<u8> # set speed percentage
SET v<u16> # set voltage in millivolts
SET a # let the device decide
```

### ADJ

ADJ (adjust) requests are made to tune the device parameters relative to the current setting.
The syntax for ADJ requests is:

`ADJ <CHAR><VALUE>`

where

-	CHAR: a one byte character, denoting the target to set. Letters must be lowercase.
-	VALUE: A numeric value, consisting of digits and (as a prefix and only once) the plus (`+`) or the minus (`-`) signs.

```
ADJ v<i16> # adjust the voltage in millivolts
ADJ %<i8> # adjust the speed percentage
```

### GET

GET requests are made to get the various readings from the operating device such as the temperature.
The syntax for GET requests is:

`GET <PARAMETER>`

where

PARAMETER: the reading or setting, ASCII string with no space characters.

```
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

Messages are terminated by a semicolon (`;`).

# Responses

There are two response types:

-	Ok: status code 0.
-	Err: status code 1.

The response format is `CODE:MSG`, where
-	CODE: the status code
-	MSG: the message string

## Sample Response

`0:voltage set to 3.3v`
