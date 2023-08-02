# GPX to Strava

This small script helps you upload gpx files in bulk to Strava. 

# Prerequisites
In order for this script to work, you need a `access_token` with `activity:write` permissions. Getting it is simple:

## 1. Create Strava API application
Create an Strava API application by visiting: https://www.strava.com/settings/api and fill out the required information.

Once done, copy the following tokens for later use:
* `client_id`
* `client_secret`

## 2. Retreive `authentication_code`
Now we need a `authentication_code` so we can exchange it for a `access_token` with write permissions. To get this we call the following URL. **Note**: be sure to replace `[client_id]` with the one from above.

```
https://www.strava.com/oauth/authorize?client_id=[client_id]&response_type=code&redirect_uri=http://localhost/exchange_token&approval_prompt=force&scope=activity:write
```

Once you open the URL you should be able to authorize your API application, and by doing so it should redirect your to a localhost URL. This URL should look something like: 

```
http://localhost/exchange_token?state=&code=24a8942e64136c&scope=activity:write
```

Copy the `code` (authentication code) from the URL, which in the above example would be `24a8942e64136c`.

## Request `access_token`

Now we need to make a POST request to get a `access_token`. Make sure to replace `[client_id]`, `[client_secret]` and `[authentication_code]`.

```
curl -X POST https://www.strava.com/api/v3/oauth/token \
    -d client_id=[client_id] \
    -d client_secret=[client_secret] \
    -d code=[authentication_code] \
    -d grant_type=authorization_code
```

If all goes well you should see a JSON response similar to below with your `access_token`.

```jsonc
{
  "token_type": "Bearer",
  "access_token": "987654321234567",
  // ..
}
```

# Running the script

```rust
cargo run -- folder_path access_token

// eg: 
// with a folder called activities
// and access_token from above
//
// cargo run -- activities 987654321234567
```

Once a GPX file has been succesfully uploaded it will be moved to `./uploaded` folder.

# Limitations
Strava has a limit of 200 requests every 15 minutes, and a maximum of 2000 requests per day.

# Getting GPX files
## Garmin Connect
https://github.com/pe-st/garmin-connect-export

# License
MIT