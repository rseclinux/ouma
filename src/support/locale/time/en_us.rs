use super::LCTime;

const TIME_EN_US: LCTime = LCTime {
  d_t_fmt: "%a %d %b %Y %r %Z",
  d_fmt: "%m/%d/%Y",
  t_fmt: "%r",
  t_fmt_ampm: "%I:%M:%S %p",
  am_str: "AM",
  pm_str: "PM",
  day: [
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday"
  ],
  abday: ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"],
  mon: [
    "January",
    "February",
    "March",
    "Apri",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December"
  ],
  abmon: [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Ju", "Aug", "Sep", "Oct", "Nov",
    "Dec"
  ]
};
