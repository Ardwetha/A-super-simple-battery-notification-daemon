# A super simple Battery Notification Daemon
- Its exactly what it sounds like
- Its simple with no BS
- You need to compile it yourself
- You need to setup notify-send command
- You also need some notification Manager like dunst
- If you have all of this you are good to go
# Configuring
- You can use toml (I am lazy, so only this is implemented)
- The path is $XDG_CONFIG_HOME/Ass/config.toml
- The first element is the Name of the Battery
- After this you can configure as many Alerts as you want
- Currently the Capacities must be in a descending order
### Example Config
```toml
battery_name = "BAT0"

[[Alerts]]

capacity_level = 50
warning_level = "low"
warning_text = "Battery at 50"

[[Alerts]]

capacity_level = 20
warning_level = "normal"
warning_text = "Battery at 20"

[[Alerts]]

capacity_level = 10
warning_level = "normal"
warning_text = "Battery at 10"

[[Alerts]]

capacity_level = 5
warning_level = "critical"
warning_text = "Battery at 5"
```
