function represent(ms) {
  return ms
    .toString(16)
    .padStart(8, "0")
    .match(/..?/g)
    .reverse()
    .map((x) => parseInt(x, 16));
}

function toMs({ hours, minutes, seconds, miliseconds }) {
  const d = new Date(hours * 1000 * 3600);
  d.setMinutes(minutes);
  d.setSeconds(seconds);
  d.setMilliseconds(miliseconds);

  return d.getTime();
}

function interpret(time) {
  const date = new Date(time);

  return {
    hours: Math.floor(time / 1000 / 3600),
    minutes: date.getMinutes(),
    seconds: date.getSeconds(),
    miliseconds: date.getMilliseconds(),
  };
}

export { represent, interpret, toMs };
