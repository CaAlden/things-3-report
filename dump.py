import things
import random
import sys

EMOJIS = ["blobcat_cookie","boat-cat","bubbles-cat","cat-confused","cat-cook","cat-on-the-laptop","cat-roll","cat-shook","cat-skype","catcat","catdance","caterpie","catjam","catsurprise","cattyping","cat_blush","cat_type","crazycat","gatocat","grumpycat","hungry_cat","nerd-cat","octocat1","octocat2","octocat3","shrodingers-cat","super_cat","surprised-cat","the-very-hungry-caterpillar","tycat","unificators","vacationing","vibingcat","villain-cat","confused-dog","cool-doge","dancing_dog","doge","dogjam","heart-eyes-dog","jakethedog1","jakethedog2","pundog","walking-the-dog","pogsire","ayaya","angryowl","audioowl","bouquet_owl","chef_owl","chocolate_owl","coffee-owl","confusedowl","coolowl","creepyowl","cupid_owl","cupid_owl_02","deadowl","dearleaderowl","dnowl","eikaiwaowl","eve-owl","eve-owl-evil","fakeowl","investigate-owl","jenkinsowl","look-owl","loopyowl","loveowl","love_letter_owl","mama-owl","mild-surprise-owl","owl-travel","owl_celebration","owl_christmas_stocking","owl_christmas_tree","owl_decorate","owl_ginger_cookie","owl_santa","owl_serious","owl_skating","owl_snowman","owl_snow_throwing","owl_toast","papa-owl","pinowl","realowl","realowl_back","realowl_guruguru","realowl_side","sadowl","sakura_owl","steampunk-owl","stopowl","surprise-owl","take-my-money-owl","tonakaiowl","valentine_gift_owl","winkowl","meow-popcorn","meowth","meow_alien","meow_angel","meow_angery2","meow_angry","meow_attention","meow_beanbag","meow_beret-coffee","meow_birthday","meow_blep","meow_bongoderp","meow_bounce","meow_brokenheart","meow_burger","meow_buzz","meow_camera","meow_code","meow_coffee","meow_coffee2","meow_comfy","meow_comfydonut","meow_comfysmirk","meow_comfy_coffee","meow_coy","meow_crazy","meow_cry","meow_dab","meow_dance","meow_dead","meow_devil-fire","meow_distrust","meow_drink","meow_dundundun","meow_dunno","meow_evil","meow_eyespin","meow_ez","meow_fat","meow_fingerguns","meow_flame_thrower","meow_flower","meow_giggle","meow_glare-zoom","meow_glowsticks","meow_googlytrash","meow_grin","meow_headache","meow_headphones","meow_heart","meow_hug","meow_hungry","meow_knife","meow_lurk","meow_mac","meow_melt","meow_mustache","meow_nix","meow_noplease","meow_nyan","meow_party","meow_pizza","meow_pop","meow_pout","meow_pressure","meow_sad","meow_sick","meow_sign","meow_sleep","meow_spy","meow_surprised","meow_sweats","meow_tableflip","meow_this","meow_thumbsdown","meow_tired","meow_wobble","meow_wow"]

target_tag = ' '.join(sys.argv[1:])
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

def format_project(project):
    return project['title']

def format_task(task):
    task_str = f"{task['title']}. {task['notes']}"
    if 'heading' in task:
        heading = things.get(task['heading'])
        task_str = f"{heading['project_title']} > {heading['title']} > {task_str}"
    elif 'project_title' in task:
        task_str = f"{task['project_title']} > {task_str}"

    return task_str.strip()


reportable = filter(lambda t: has_tag(t, target_tag), things.today())

output = [' '.join(map(lambda e: f':{e}:', random.choices(EMOJIS, k=3))), ''] # TODO: Three random emojis?

for reported in reportable:
    if reported['type'] == 'project':
        output.append(f'- {format_project(reported)}')
    else:
        output.append(f'- {format_task(reported)}')

print('\n'.join(output))
