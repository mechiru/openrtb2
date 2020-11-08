/// 5.1 Content Categories
///
/// The following list represents the IAB’s contextual taxonomy for categorization. Standard IDs
/// have been adopted to easily support the communication of primary and secondary categories for
/// various objects. This OpenRTB table has values derived from the IAB Tech Lab Content Taxonomy.
/// Practitioners should keep in sync with updates as published on www.iab.com.
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq, Clone, Copy)]
#[serde(rename_all = "SCREAMING-KEBAB-CASE")]
pub enum ContentCategory {
    /// Arts & Entertainment
    Iab1,
    /// Books & Literature
    Iab1_1,
    /// Celebrity Fan/Gossip
    Iab1_2,
    /// Fine Art
    Iab1_3,
    /// Humor
    Iab1_4,
    /// Movies
    Iab1_5,
    /// Music
    Iab1_6,
    /// Television
    Iab1_7,
    /// Automotive
    Iab2,
    /// Auto Parts
    Iab2_1,
    /// Auto Repair
    Iab2_2,
    /// Buying/Selling Cars
    Iab2_3,
    /// Car Culture
    Iab2_4,
    /// Certified Pre_Owned
    Iab2_5,
    /// Convertible
    Iab2_6,
    /// Coupe
    Iab2_7,
    /// Crossover
    Iab2_8,
    /// Diesel
    Iab2_9,
    /// Electric Vehicle
    Iab2_10,
    /// Hatchback
    Iab2_11,
    /// Hybrid
    Iab2_12,
    /// Luxury
    Iab2_13,
    /// Minivan
    Iab2_14,
    /// Motorcycles
    Iab2_15,
    /// Off_Road Vehicles
    Iab2_16,
    /// Performance Vehicles
    Iab2_17,
    /// Pickup
    Iab2_18,
    /// Road-Side Assistance
    Iab2_19,
    /// Sedan
    Iab2_20,
    /// Trucks & Accessories
    Iab2_21,
    /// Vintage Cars
    Iab2_22,
    /// Wagon
    Iab2_23,
    /// Business
    Iab3,
    /// Advertising
    Iab3_1,
    /// Agriculture
    Iab3_2,
    /// Biotech/Biomedical
    Iab3_3,
    /// Business Software
    Iab3_4,
    /// Construction
    Iab3_5,
    /// Forestry
    Iab3_6,
    /// Government
    Iab3_7,
    /// Green Solutions
    Iab3_8,
    /// Human Resources
    Iab3_9,
    /// Logistics
    Iab3_10,
    /// Marketing
    Iab3_11,
    /// Metals
    Iab3_12,
    /// Careers
    Iab4,
    /// Career Planning
    Iab4_1,
    /// College
    Iab4_2,
    /// Financial Aid
    Iab4_3,
    /// Job Fairs
    Iab4_4,
    /// Job Search
    Iab4_5,
    /// Resume Writing/Advice
    Iab4_6,
    /// Nursing
    Iab4_7,
    /// Scholarships
    Iab4_8,
    /// Telecommuting
    Iab4_9,
    /// U.S. Military
    Iab4_10,
    /// Career Advice
    Iab4_11,
    /// Education
    Iab5,
    /// 7-12 Education
    Iab5_1,
    /// Adult Education
    Iab5_2,
    /// Art History
    Iab5_3,
    /// College Administration
    Iab5_4,
    /// College Life
    Iab5_5,
    /// Distance Learning
    Iab5_6,
    /// English as a 2nd Language
    Iab5_7,
    /// Language Learning
    Iab5_8,
    /// Graduate School
    Iab5_9,
    /// Homeschooling
    Iab5_10,
    /// Homework/Study Tips
    Iab5_11,
    /// K-6 Educators
    Iab5_12,
    /// Private School
    Iab5_13,
    /// Special Education
    Iab5_14,
    /// Studying Business
    Iab5_15,
    /// Family & Parenting
    Iab6,
    /// Adoption
    Iab6_1,
    /// Babies & Toddlers
    Iab6_2,
    /// Daycare/Pre School
    Iab6_3,
    /// Family Internet
    Iab6_4,
    /// Parenting - K-6 Kids
    Iab6_5,
    /// Parenting teens
    Iab6_6,
    /// Pregnancy
    Iab6_7,
    /// Special Needs Kids
    Iab6_8,
    /// Eldercare
    Iab6_9,
    /// Health & Fitness
    Iab7,
    /// Exercise
    Iab7_1,
    /// ADD
    Iab7_2,
    /// AIDS/HIV
    Iab7_3,
    /// Allergies
    Iab7_4,
    /// Alternative Medicine
    Iab7_5,
    /// Arthritis
    Iab7_6,
    /// Asthma
    Iab7_7,
    /// Autism/PDD
    Iab7_8,
    /// Bipolar Disorder
    Iab7_9,
    /// Brain Tumor
    Iab7_10,
    /// Cancer
    Iab7_11,
    /// Cholesterol
    Iab7_12,
    /// Chronic Fatigue Syndrome
    Iab7_13,
    /// Chronic Pain
    Iab7_14,
    /// Cold & Flu
    Iab7_15,
    /// Deafness
    Iab7_16,
    /// Dental Care
    Iab7_17,
    /// Depression
    Iab7_18,
    /// Dermatology
    Iab7_19,
    /// Diabetes
    Iab7_20,
    /// Epilepsy
    Iab7_21,
    /// GERD/Acid Reflux
    Iab7_22,
    /// Headaches/Migraines
    Iab7_23,
    /// Heart Disease
    Iab7_24,
    /// Herbs for Health
    Iab7_25,
    /// Holistic Healing
    Iab7_26,
    /// IBS/Crohn’s Disease
    Iab7_27,
    /// Incest/Abuse Support
    Iab7_28,
    /// Incontinence
    Iab7_29,
    /// Infertility
    Iab7_30,
    /// Men’s Health
    Iab7_31,
    /// Nutrition
    Iab7_32,
    /// Orthopedics
    Iab7_33,
    /// Panic/Anxiety Disorders
    Iab7_34,
    /// Pediatrics
    Iab7_35,
    /// Physical Therapy
    Iab7_36,
    /// Psychology/Psychiatry
    Iab7_37,
    /// Senior Health
    Iab7_38,
    /// Sexuality
    Iab7_39,
    /// Sleep Disorders
    Iab7_40,
    /// Smoking Cessation
    Iab7_41,
    /// Substance Abuse
    Iab7_42,
    /// Thyroid Disease
    Iab7_43,
    /// Weight Loss
    Iab7_44,
    /// Women's Health
    Iab7_45,
    /// Food & Drink
    Iab8,
    /// American Cuisine
    Iab8_1,
    /// Barbecues & Grilling
    Iab8_2,
    /// Cajun/Creole
    Iab8_3,
    /// Chinese Cuisine
    Iab8_4,
    /// Cocktails/Beer
    Iab8_5,
    /// Coffee/Tea
    Iab8_6,
    /// Cuisine-Specific
    Iab8_7,
    /// Desserts & Baking
    Iab8_8,
    /// Dining Out
    Iab8_9,
    /// Food Allergies
    Iab8_10,
    /// French Cuisine
    Iab8_11,
    /// Health/Low-Fat Cooking
    Iab8_12,
    /// Italian Cuisine
    Iab8_13,
    /// Japanese Cuisine
    Iab8_14,
    /// Mexican Cuisine
    Iab8_15,
    /// Vegan
    Iab8_16,
    /// Vegetarian
    Iab8_17,
    /// Wine
    Iab8_18,
    /// Hobbies & Interests
    Iab9,
    /// Art/Technology
    Iab9_1,
    /// Arts & Crafts
    Iab9_2,
    /// Beadwork
    Iab9_3,
    /// Bird-Watching
    Iab9_4,
    /// Board Games/Puzzles
    Iab9_5,
    /// Candle & Soap Making
    Iab9_6,
    /// Card Games
    Iab9_7,
    /// Chess
    Iab9_8,
    /// Cigars
    Iab9_9,
    /// Collecting
    Iab9_10,
    /// Comic Books
    Iab9_11,
    /// Drawing/Sketching
    Iab9_12,
    /// Freelance Writing
    Iab9_13,
    /// Genealogy
    Iab9_14,
    /// Getting Published
    Iab9_15,
    /// Guitar
    Iab9_16,
    /// Home Recording
    Iab9_17,
    /// Investors & Patents
    Iab9_18,
    /// Jewelry Making
    Iab9_19,
    /// Magic & Illusion
    Iab9_20,
    /// Needlework
    Iab9_21,
    /// Painting
    Iab9_22,
    /// Photography
    Iab9_23,
    /// Radio
    Iab9_24,
    /// Roleplaying Games
    Iab9_25,
    /// Sci-Fi & Fantasy
    Iab9_26,
    /// Scrapbooking
    Iab9_27,
    /// Screenwriting
    Iab9_28,
    /// Stamps & Coins
    Iab9_29,
    /// Video & Computer Games
    Iab9_30,
    /// Woodworking
    Iab9_31,
    /// Home & Garden
    Iab10,
    /// Appliances
    Iab10_1,
    /// Entertaining
    Iab10_2,
    /// Environmental Safety
    Iab10_3,
    /// Gardening
    Iab10_4,
    /// Home Repair
    Iab10_5,
    /// Home Theater
    Iab10_6,
    /// Interior Decorating
    Iab10_7,
    /// Landscaping
    Iab10_8,
    /// Remodeling & Construction
    Iab10_9,
    /// Law, Government, & Politics
    Iab11,
    /// Immigration
    Iab11_1,
    /// Legal Issues
    Iab11_2,
    /// U.S. Government Resources
    Iab11_3,
    /// Politics
    Iab11_4,
    /// Commentary
    Iab11_5,
    /// News
    Iab12,
    /// International News
    Iab12_1,
    /// National News
    Iab12_2,
    /// Local News
    Iab12_3,
    /// Personal Finance
    Iab13,
    /// Beginning Investing
    Iab13_1,
    /// Credit/Debt & Loans
    Iab13_2,
    /// Financial News
    Iab13_3,
    /// Financial Planning
    Iab13_4,
    /// Hedge Fund
    Iab13_5,
    /// Insurance
    Iab13_6,
    /// Investing
    Iab13_7,
    /// Mutual Funds
    Iab13_8,
    /// Options
    Iab13_9,
    /// Retirement Planning
    Iab13_10,
    /// Stocks
    Iab13_11,
    /// Tax Planning
    Iab13_12,
    /// Society
    Iab14,
    /// Dating
    Iab14_1,
    /// Divorce Support
    Iab14_2,
    /// Gay Life
    Iab14_3,
    /// Marriage
    Iab14_4,
    /// Senior Living
    Iab14_5,
    /// Teens
    Iab14_6,
    /// Weddings
    Iab14_7,
    /// Ethnic Specific
    Iab14_8,
    /// Science
    Iab15,
    /// Astrology
    Iab15_1,
    /// Biology
    Iab15_2,
    /// Chemistry
    Iab15_3,
    /// Geology
    Iab15_4,
    /// Paranormal Phenomena
    Iab15_5,
    /// Physics
    Iab15_6,
    /// Space/Astronomy
    Iab15_7,
    /// Geography
    Iab15_8,
    /// Botany
    Iab15_9,
    /// Weather
    Iab15_10,
    /// Pets
    Iab16,
    /// Aquariums
    Iab16_1,
    /// Birds
    Iab16_2,
    /// Cats
    Iab16_3,
    /// Dogs
    Iab16_4,
    /// Large Animals
    Iab16_5,
    /// Reptiles
    Iab16_6,
    /// Veterinary Medicine
    Iab16_7,
    /// Sports
    Iab17,
    /// Auto Racing
    Iab17_1,
    /// Baseball
    Iab17_2,
    /// Bicycling
    Iab17_3,
    /// Bodybuilding
    Iab17_4,
    /// Boxing
    Iab17_5,
    /// Canoeing/Kayaking
    Iab17_6,
    /// Cheerleading
    Iab17_7,
    /// Climbing
    Iab17_8,
    /// Cricket
    Iab17_9,
    /// Figure Skating
    Iab17_10,
    /// Fly Fishing
    Iab17_11,
    /// Football
    Iab17_12,
    /// Freshwater Fishing
    Iab17_13,
    /// Game & Fish
    Iab17_14,
    /// Golf
    Iab17_15,
    /// Horse Racing
    Iab17_16,
    /// Horses
    Iab17_17,
    /// Hunting/Shooting
    Iab17_18,
    /// Inline Skating
    Iab17_19,
    /// Martial Arts
    Iab17_20,
    /// Mountain Biking
    Iab17_21,
    /// NASCAR Racing
    Iab17_22,
    /// Olympics
    Iab17_23,
    /// Paintball
    Iab17_24,
    /// Power & Motorcycles
    Iab17_25,
    /// Pro Basketball
    Iab17_26,
    /// Pro Ice Hockey
    Iab17_27,
    /// Rodeo
    Iab17_28,
    /// Rugby
    Iab17_29,
    /// Running/Jogging
    Iab17_30,
    /// Sailing
    Iab17_31,
    /// Saltwater Fishing
    Iab17_32,
    /// Scuba Diving
    Iab17_33,
    /// Skateboarding
    Iab17_34,
    /// Skiing
    Iab17_35,
    /// Snowboarding
    Iab17_36,
    /// Surfing/Body-Boarding
    Iab17_37,
    /// Swimming
    Iab17_38,
    /// Table Tennis/Ping-Pong
    Iab17_39,
    /// Tennis
    Iab17_40,
    /// Volleyball
    Iab17_41,
    /// Walking
    Iab17_42,
    /// Waterski/Wakeboard
    Iab17_43,
    /// World Soccer
    Iab17_44,
    /// Style & Fashion
    Iab18,
    /// Beauty
    Iab18_1,
    /// Body Art
    Iab18_2,
    /// Fashion
    Iab18_3,
    /// Jewelry
    Iab18_4,
    /// Clothing
    Iab18_5,
    /// Accessories
    Iab18_6,
    /// Technology & Computing
    Iab19,
    /// 3-D Graphics
    Iab19_1,
    /// Animation
    Iab19_2,
    /// Antivirus Software
    Iab19_3,
    /// C/C++
    Iab19_4,
    /// Cameras & Camcorders
    Iab19_5,
    /// Cell Phones
    Iab19_6,
    /// Computer Certification
    Iab19_7,
    /// Computer Networking
    Iab19_8,
    /// Computer Peripherals
    Iab19_9,
    /// Computer Reviews
    Iab19_10,
    /// Data Centers
    Iab19_11,
    /// Databases
    Iab19_12,
    /// Desktop Publishing
    Iab19_13,
    /// Desktop Video
    Iab19_14,
    /// Email
    Iab19_15,
    /// Graphics Software
    Iab19_16,
    /// Home Video/DVD
    Iab19_17,
    /// Internet Technology
    Iab19_18,
    /// Java
    Iab19_19,
    /// JavaScript
    Iab19_20,
    /// Mac Support
    Iab19_21,
    /// MP3/MIDI
    Iab19_22,
    /// Net Conferencing
    Iab19_23,
    /// Net for Beginners
    Iab19_24,
    /// Network Security
    Iab19_25,
    /// Palmtops/PDAs
    Iab19_26,
    /// PC Support
    Iab19_27,
    /// Portable
    Iab19_28,
    /// Entertainment
    Iab19_29,
    /// Shareware/Freeware
    Iab19_30,
    /// Unix
    Iab19_31,
    /// Visual Basic
    Iab19_32,
    /// Web Clip Art
    Iab19_33,
    /// Web Design/HTML
    Iab19_34,
    /// Web Search
    Iab19_35,
    /// Windows
    Iab19_36,
    /// Travel
    Iab20,
    /// Adventure Travel
    Iab20_1,
    /// Africa
    Iab20_2,
    /// Air Travel
    Iab20_3,
    /// Australia & New Zealand
    Iab20_4,
    /// Bed & Breakfasts
    Iab20_5,
    /// Budget Travel
    Iab20_6,
    /// Business Travel
    Iab20_7,
    /// By US Locale
    Iab20_8,
    /// Camping
    Iab20_9,
    /// Canada
    Iab20_10,
    /// Caribbean
    Iab20_11,
    /// Cruises
    Iab20_12,
    /// Eastern Europe
    Iab20_13,
    /// Europe
    Iab20_14,
    /// France
    Iab20_15,
    /// Greece
    Iab20_16,
    /// Honeymoons/Getaways
    Iab20_17,
    /// Hotels
    Iab20_18,
    /// Italy
    Iab20_19,
    /// Japan
    Iab20_20,
    /// Mexico & Central America
    Iab20_21,
    /// National Parks
    Iab20_22,
    /// South America
    Iab20_23,
    /// Spas
    Iab20_24,
    /// Theme Parks
    Iab20_25,
    /// Traveling with Kids
    Iab20_26,
    /// United Kingdom
    Iab20_27,
    /// Real Estate
    Iab21,
    /// Apartments
    Iab21_1,
    /// Architects
    Iab21_2,
    /// Buying/Selling Homes
    Iab21_3,
    /// Shopping
    Iab22,
    /// Contests & Freebies
    Iab22_1,
    /// Couponing
    Iab22_2,
    /// Comparison
    Iab22_3,
    /// Engines
    Iab22_4,
    /// Religion & Spirituality
    Iab23,
    /// Alternative Religions
    Iab23_1,
    /// Atheism/Agnosticism
    Iab23_2,
    /// Buddhism
    Iab23_3,
    /// Catholicism
    Iab23_4,
    /// Christianity
    Iab23_5,
    /// Hinduism
    Iab23_6,
    /// Islam
    Iab23_7,
    /// Judaism
    Iab23_8,
    /// Latter-Day Saints
    Iab23_9,
    /// Pagan/Wiccan
    Iab23_10,
    /// Uncategorized
    Iab24,
    /// Non-Standard Content
    Iab25,
    /// Unmoderated UGC
    Iab25_1,
    /// Extreme Graphic/Explicit Violence
    Iab25_2,
    /// Pornography
    Iab25_3,
    /// Profane Content
    Iab25_4,
    /// Hate Content
    Iab25_5,
    /// Under Construction
    Iab25_6,
    /// Incentivized
    Iab25_7,
    /// Illegal Content
    Iab26,
    /// Illegal Content
    Iab26_1,
    /// Warez
    Iab26_2,
    /// Spyware/Malware
    Iab26_3,
    /// Copyright Infringement
    Iab26_4,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn json() -> serde_json::Result<()> {
        assert!(serde_json::from_str::<ContentCategory>("IAB").is_err());

        let json = r#"["IAB1","IAB1-1"]"#;
        let e1: Vec<ContentCategory> = serde_json::from_str(json)?;
        assert_eq!(e1, vec![ContentCategory::Iab1, ContentCategory::Iab1_1]);
        assert_eq!(serde_json::to_string(&e1)?, json);

        Ok(())
    }
}
