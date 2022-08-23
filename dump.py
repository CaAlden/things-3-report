import things
import argparse
import re
import random
import sys

with open('./emojis.txt', 'r') as emoji_file:
    EMOJIS = [l.strip() for l in emoji_file.readlines()]

def mine_tags(tags, place):
    if place is None or not 'tags' in place:
        return tags

    tags.extend(place['tags'])

def has_tag(task, tag):
    area = things.areas(uuid=task['area']) if 'area' in task else None
    project = things.projects(uuid=task['project']) if 'project' in task else None
    heading_project = things.projects(uuid=things.get(uuid=task['heading'])['project']) if 'heading' in task else None
    project_area = things.areas(uuid=project['area']) if project is not None and 'area' in project else None
    tags = []
    mine_tags(tags, task)
    mine_tags(tags, area)
    mine_tags(tags, heading_project)
    mine_tags(tags, project)
    mine_tags(tags, project_area)
    return tag in tags

def tasks_to_heirarchy(top_level_comment, tasks):
    heir = {}
    for task in tasks:
        task_leaf = (task, {})
        if 'project' not in task and 'heading' not in task:
            heir[task['uuid']] = task_leaf
        elif 'heading' in task:
            h_uuid = task['heading']
            head = things.get(h_uuid)

            p_uuid = head['project'] # This must exist
            proj = things.get(p_uuid)
            proj_title, proj_sublevel = heir[p_uuid] if p_uuid in heir else (proj, {})
            head_title, head_sublevel = proj_sublevel[h_uuid] if h_uuid in proj_sublevel else (head, {})
            head_sublevel[task['uuid']] = task_leaf
            proj_sublevel[head['uuid']] = (head_title, head_sublevel)
            heir[p_uuid] = (proj_title, proj_sublevel)
        else: # The project must not be None here
            p_uuid = task['project']
            proj = things.get(p_uuid)
            proj_title, proj_sublevel = heir[p_uuid] if p_uuid in heir else (proj, {})
            proj_sublevel[task['uuid']] = task_leaf
            heir[p_uuid] = (proj_title, proj_sublevel)

    return (top_level_comment, heir)

def parse_note(note):
    note_block_pattern = r"```report\n([\s\S]*?)\n?```"
    return ' '.join(re.findall(note_block_pattern, note))

def make_recursive_formatter(formatter):
    def recursive_format(structure, depth):
        (node, branches) = structure
        notes = parse_note(node['notes'])
        node_str = formatter.node(node, notes)
        if len(branches) == 1:
            item = list(branches.values())[0]
            node_str += formatter.single(recursive_format(item, depth))
        elif len(branches) > 1:
            new_depth = depth + 2
            bullets = [formatter.bullet(f"{recursive_format(item, new_depth)}", new_depth) for item in branches.values()]
            node_str += ''.join(map(lambda b: f'\n{b}', bullets))

        return node_str
    return recursive_format

class TodayFormatter:
    def node(self, node, notes):
        return f"{node['title']}{'' if notes == '' else f' {notes}'}"

    def single(self, subtext):
        return f" > {subtext}"

    def bullet(self, subtext, depth):
        space = ''.join([' '] * depth)
        return f"{space}- {subtext}"

class LogbookFormatter:
    def node(self, node, notes):
        node_text = f"{node['title']}{'' if notes == '' or node['status'] == 'canceled' else f' {notes}'}"
        if node['status'] == 'canceled':
            node_text = f"~{node_text}~"

        return node_text

    def single(self, subtext):
        return f" > {subtext}"

    def bullet(self, subtext, depth):
        space = ''.join([' '] * depth)
        return f"{space}- {subtext}"


def generate_signoff_message(target_tag):
    format_tasks = make_recursive_formatter(LogbookFormatter())
    import datetime
    today_str = str(datetime.datetime.today()).split()[0]
    reportable = list(filter(lambda t: has_tag(t, target_tag) and t['stop_date'] == today_str and t['status'] == 'completed', things.logbook()))
    structured_tasks = tasks_to_heirarchy({ 'title': 'Stopping now', 'notes': '', 'status': '' }, reportable)
    return format_tasks(structured_tasks, 0)

def generate_today_message(target_tag):
    format_tasks = make_recursive_formatter(TodayFormatter())
    reportable = list(filter(lambda t: has_tag(t, target_tag), things.today()))

    top_level_comment = ' '.join(map(lambda e: f':{e}:', random.choices(EMOJIS, k=3)))
    structured_tasks = tasks_to_heirarchy({ 'title': top_level_comment, 'notes': '' }, reportable)
    return format_tasks(structured_tasks, 0)

if __name__ == '__main__':
    parser = argparse.ArgumentParser(description="Generate iknow_vacation_remote style messages from Things 3 Tasks")
    parser.add_argument('tags', metavar='TAG', type=str, nargs="+", help="a word in the overall tag")
    parser.add_argument('--signoff', action=argparse.BooleanOptionalAction, help="Get tasks from the logbook and generate output for a signoff message")
    args = parser.parse_args()
    target_tag = ' '.join(args.tags)

    if args.signoff:
        print(generate_signoff_message(target_tag))
    else:
        print(generate_today_message(target_tag))


