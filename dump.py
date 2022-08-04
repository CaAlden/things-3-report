import things
import argparse
import re
import random
import sys

EMOJIS = ["blobcat_cookie","boat-cat","bubbles-cat","cat-confused","cat-cook","cat-on-the-laptop","cat-roll","cat-shook","cat-skype","catcat","catdance","caterpie","catjam","catsurprise","cattyping","cat_blush","cat_type","crazycat","gatocat","grumpycat","hungry_cat","nerd-cat","octocat1","octocat2","octocat3","shrodingers-cat","super_cat","surprised-cat","the-very-hungry-caterpillar","tycat","vacationing","vibingcat","villain-cat","confused-dog","cool-doge","dancing_dog","doge","dogjam","heart-eyes-dog","jakethedog1","jakethedog2","pundog","walking-the-dog","pogsire","ayaya","angryowl","audioowl","bouquet_owl","chef_owl","chocolate_owl","coffee-owl","confusedowl","coolowl","creepyowl","cupid_owl","cupid_owl_02","deadowl","dearleaderowl","dnowl","eikaiwaowl","eve-owl","eve-owl-evil","fakeowl","investigate-owl","jenkinsowl","look-owl","loopyowl","loveowl","love_letter_owl","mama-owl","mild-surprise-owl","owl-travel","owl_celebration","owl_christmas_stocking","owl_christmas_tree","owl_decorate","owl_ginger_cookie","owl_santa","owl_serious","owl_skating","owl_snowman","owl_snow_throwing","owl_toast","papa-owl","pinowl","realowl","realowl_back","realowl_guruguru","realowl_side","sadowl","sakura_owl","steampunk-owl","stopowl","surprise-owl","take-my-money-owl","tonakaiowl","valentine_gift_owl","winkowl","meow-popcorn","meowth","meow_alien","meow_angel","meow_angery2","meow_angry","meow_attention","meow_beanbag","meow_beret-coffee","meow_birthday","meow_blep","meow_bongoderp","meow_bounce","meow_brokenheart","meow_burger","meow_buzz","meow_camera","meow_code","meow_coffee","meow_coffee2","meow_comfy","meow_comfydonut","meow_comfysmirk","meow_comfy_coffee","meow_coy","meow_crazy","meow_cry","meow_dab","meowrainjoy","meow_dance","meow_dead","meow_devil-fire","meow_distrust","meow_drink","meow_dundundun","meow_dunno","meow_evil","meow_eyespin","meow_ez","meow_fat","meow_fingerguns","meow_flame_thrower","meow_flower","meow_giggle","meow_glare-zoom","meow_glowsticks","meow_googlytrash","meow_grin","meow_headache","meow_headphones","meow_heart","meow_hug","meow_hungry","meow_knife","meow_lurk","meow_mac","meow_melt","meow_mustache","meow_nix","meow_noplease","meow_nyan","meow_party","meow_pizza","meow_pop","meow_pout","meow_pressure","meow_sad","meow_sick","meow_sign","meow_sleep","meow_spy","meow_surprised","meow_sweats","meow_tableflip","meow_this","meow_thumbsdown","meow_tired","meow_wobble","meow_wow"]

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
        if node['status'] == 'completed':
            node_text = f":white_check_mark: {node_text}"
        elif node['status'] == 'canceled':
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
    reportable = list(filter(lambda t: has_tag(t, target_tag) and t['stop_date'] == today_str, things.logbook()))
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


