var things = Application("Things");
var logbook = things.lists.byName("Logbook").toDos();
var objs = [];

var from = new Date($params.from);
var to = new Date($params.to);

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
