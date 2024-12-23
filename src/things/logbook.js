var things = Application("Things");
var logbook = things.lists.byName("Logbook").toDos();
var objs = [];

var from = new Date();
from.setHours(0);
from.setMinutes(0);
from.setSeconds(0);
var to = new Date();
to.setTime(from.getTime());
to.setHours(23);
to.setMinutes(59);
to.setSeconds(59);

for (const todo of logbook) {
  if(todo.completionDate() >= from && todo.completionDate() < to) {
    var proj = todo.project();
    var tags = [];
    if (proj) {
      tags.push(...proj.tagNames().split(', '));
    }
    var area = todo.area() || proj && proj.area();
    objs.push({
      id: todo.id(),
      title: todo.name(),
      notes: todo.notes() || null,
      status: todo.status(),
      completion_date: todo.completionDate(),
      project: proj && {
        id: proj.id(),
        title: proj.name(),
        status: proj.status(),
        notes: proj.notes(),
        tags: proj.tagNames().split(', '),
      },
      area: area && { id: area.id(), title: area.name() },
      tags: [...tags, ...todo.tagNames().split(', ')].filter(t => t),
    });
  } else {
    break;
  }
}

return JSON.stringify(objs, undefined, 2);
