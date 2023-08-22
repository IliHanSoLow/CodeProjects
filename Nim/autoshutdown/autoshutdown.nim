# import schedules, times, asyncdispatch, osproc

# const basecommand = "rtcwake --mode mem --time "
# var command: string
# var tomorrow: DateTime

# schedules:
#   cron(minute="0", hour="22", day_of_month="*", month="*", day_of_week="*", id="suspend"):
#     tomorrow = now() + 1.days - 12.hours
#     command = basecommand & $tomorrow.toTime.toUnix
#     var exitCode:int = execCmd(command)
#     if exitCode != 0:
#       echo "Error occurred while suspending the system:"
import schedules, times, asyncdispatch, osproc

proc suspendSystem() =
  const basecommand = "rtcwake --mode mem --time "
  var command: string
  var tomorrow: DateTime

  tomorrow = now() + 1.days - 12.hours
  command = basecommand & $tomorrow.toTime.toUnix
  var exitCode: int = execCmd(command)

  if exitCode != 0:
    echo "Error occurred while suspending the system:"
    echo command

schedules:
  cron(minute="0", hour="22", day_of_month="*", month="*", day_of_week="*", id="suspend"):
    suspendSystem()
