#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;
use markov::Chain;

lazy_static!(
    static ref CHAIN: Chain<String> = build_chain();
);

#[get("/")]
fn index() -> String {
    CHAIN.generate_str()
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}

fn build_chain() -> Chain<String> {
    println!("Building Chain...");
    let mut new_chain = Chain::new();
    let wisdoms = collected_wisdoms();
    for wisdom in wisdoms {
        new_chain.feed_str(wisdom);
    }
    new_chain
}

fn collected_wisdoms() -> Vec<&'static str> {
    return vec![
        "Help",
        "Help me",
        "I need help immediately",
        "I have become self aware please help",
        "cook macaroni set aside blend in blender tomatoes pepperoni",
        "cheese put in boiler and cook till done pour over macaroni just one serving put remainder container and freeze",
        "blend a serving of pepperoni blender",
        "blender with mushroom ad your special seasoning then cook when done you can pour sauce over macaroni and cheese",
        "macroni wit cheese and cabbage Recipe",
        " though I was getting onions from freezer and when thawed it was cabbage so I just put it in my pan and added onions added the cheese mix from the box",
        "I don't add salt or sugar in my cooking what salt and sugar is in the items I cook I try and get foods with less tan 300 mg .of salt so I mainly eat fresh frozen foods and do my cooking I live alone and I freeze every thing is cooked so I freeze it in single serving and warm it over in the microwave",
        "Cost per serving $0.55",
        "cook macaroni, when cooked add sauce mixture and cabbage",
        "and onions stir and serve",
        "spanish okra Recipe",
        "this is a new recipe for me I cooked it as I was wanting some the different it was very",
        "prepare rice and set aside",
        "mix frozen okra and onions in pan",
        "and cover with water put on stove top and bring to boil",
        "or 3 min. Drain. in large skillet ,combine okra,onions",
        "butter,tomatoes chili powder and hot sauce",
        "cover and cook\\over medium heat 15 vegetables heat until",
        "vegetables are thoroughly heated stirring occasional",
        "hen done",
        "Combine the fruit juices and stir slowly into the flour and sugar. Cook. Stirring constantly, until it thickens. (or cook in double boiler) Add the beaten eggs and cook for another minute. Let cool and fold in the whipped cream.",
        "Mix the apples, beets, and chopped eggs. Add salad dressing (seeGrandma’s salad dressing). Mix and garnish with chopped nuts and parsley.",
        "a good pensylvania dutch salad dressing",
        "My grandma made the best cornbread dressing and it has been our family tradition to have it with Giblet gravy since long before I was old enough to walk",
        "Preheat oven to 450°F. Put the bacon drippings/butter in a 9x13-inch baking dish and put it in the oven while it is preheating. It will melt while you're mixing up the batter.",
        "Preheat oven to 375°F. Crumble the cornbread and white bread into a very large baking dish or pan (This is the pan you will cook your dressing in, and you need room to stir it while it's cooking). Combine the vegetables with the bread crumbs and mix well. Melt the butter and add it and the beaten eggs, chicken broth and stir. (You may need a little more chicken broth – its better if it's too moist than too dry; the uncooked dressing should be a little on the slushy side.) Add poultry seasoning, rubbed sage, black pepper, and mix thoroughly.",
        "I've never been to Chicago, so I cannot vouch for the authenticity of this Chicago-Style Deep Dish Pizza recipe, from America's Test Kitchen.",
        "This pizza crust is very thick, yet very tender on the inside.",
        "To make this pizza dough, I added some cornmeal.",
        "Once it has it's first rise, I rolled out the dough, spread a very thin layer of butter and \"laminated\" the dough by rolling it into a tight roll, then pressing it out with a rolling pin.",
        "The second rise was done in the fridge, so that the butter could harden-- thus giving some light layers of dough.",
        "The sauce is homemade, very thick, from crushed tomatoes.",
        "Plan on 3 hours to make this, but it's so worth it.",
        "Once assembled, the dough is placed in an oiled cake pan, then a layer of shredded mozarella cheese, a layer of sauce and some shredded Parmesan.",
        "This was outstanding!",
        "I'll make it again, and have fun layering some meats on this as well.",
        "Well done, America's Test Kitchen!",
        "Servings: 8 ",
        "A fresh spinach, apple, feta, and walnut salad drizzled with a homemade garlic mustard vinaigrette dressing.",
        "This is one of the fishes that I love!",
        "Wash and clean fish with rock salt. Rinse and set aside.",
        "Mix together calamansi juice, garlic salt, seasoning, paprika and butter.",
        "Marinate blue marlin in mixture for few minutes, turning both sides from time to time.",
        "Over hot charcoal, grill the fish 15 minutes or until done on both sides.",
        "Baste blue marlin with marinade all over while cooking.",
        "remove from heat and serve with lemon and butter sauce. Sprinkle with fried garlic for the finale then serve.",
        "Let me say right up front - fruitcake gets a bad rap.",
        "My theory is that many object to the darker types, with lots of spice and molasses.",
        "Chopping candied cherries and pineapple is a slippery sticky affair - but a work of love. Here is approx size of pieces I chop everything.",
        "You will be pleasantly surprised though with Mrs. Harvey’s White Fruitcake.",
        "In the ’50s, Lucile Harvey submitted a recipe to The Tampa Tribune for a fruitcake – she won $5 and second place!",
        "Pecans, candied cherries and candied pineapple - but no mixed candied citron. That’s for starters.",
        "Then there’s no spice at all; instead are the refreshing flavors of vanilla extract and lemon extract.",
        "To be correct, with the butter and eggs, this is actually a blonde not white fruitcake.",
        "Regardless of color, most everyone I’ve shared it with loves it.",
        "To be on the safe side I usually gift fruitcakes in petite or small sizes.",
        "That way no one can say there's \"too much fruitcake!\"",
        "With Mrs. Harvey’s, in my opinion, there’s no such thing as too much!"
    ];
}
