use std::collections::HashMap;

macro_rules! help {
	($args:expr) => {{
		println!("{} day part inputs\n\tday\t01..25\n\tpart = 1 or 2", $args[0]);
		println!("\tinputs: test1 test2 test3 input");
		return
	}}
}

macro_rules! trace_wire_v1 {
	($panel:expr, $coords:expr, $wire_id:expr) => {
		let val = $panel.entry($coords).or_insert("".to_owned());
		let mut v = String::new();
		v.push_str(val);
		v.push_str($wire_id);
		$panel.insert($coords, v);
	};
}
pub fn part1(args: Vec<String>) {
    println!("--- Day 3: Crossed Wires ---");
    println!("--- Part 1");

    if args.len() != 4 { help!(args); }

	let mut paths: Vec<(&str,Vec<(&str, usize)>)> = Vec::new();
	match args[3].as_ref() {
		// inputs test 1 => 6
		"test1" => {
			paths.push(("a",vec![("R",8),("U",5),("L",5),("D",3)]));
			paths.push(("b",vec![("U",7),("R",6),("D",4),("L",4)]));
		},
		// inputs test 2 => 159
		"test2" => {
			paths.push(("a",vec![("R",75),("D",30),("R",83),("U",83),("L",12),("D",49),("R",71),("U",7),("L",72)]));
			paths.push(("b",vec![("U",62),("R",66),("U",55),("R",34),("D",71),("R",55),("D",58),("R",83)]));
		},
		// inputs test 3 => 135
		"test3" => {
			paths.push(("a",vec![("R",98),("U",47),("R",26),("D",63),("R",33),("U",87),("L",62),("D",20),("R",33),("U",53),("R",51)]));
			paths.push(("b",vec![("U",98),("R",91),("D",20),("R",16),("D",67),("R",40),("U",7),("R",15),("U",6),("R",7)]));
		},
		// my inputs 
		"input" => {
			paths.push(("a",vec![("R",998),("U",367),("R",735),("U",926),("R",23),("U",457),("R",262),("D",473),("L",353),("U",242),("L",930),("U",895),("R",321),("U",683),("L",333),("U",623),("R",105),("D",527),("R",437),("D",473),("L",100),("D",251),("L",958),("U",384),("R",655),("U",543),("L",704),("D",759),("R",529),("D",176),("R",835),("U",797),("R",453),("D",650),("L",801),("U",437),("L",468),("D",841),("R",928),("D",747),("L",803),("U",677),("R",942),("D",851),("R",265),("D",684),("L",206),("U",763),("L",566),("U",774),("L",517),("U",337),("L",86),("D",585),("R",212),("U",656),("L",799),("D",953),("L",24),("U",388),("L",465),("U",656),("L",467),("U",649),("R",658),("U",519),("L",966),("D",290),("L",979),("D",819),("R",208),("D",907),("R",941),("D",458),("L",882),("U",408),("R",539),("D",939),("R",557),("D",771),("L",448),("U",460),("L",586),("U",148),("R",678),("U",360),("R",715),("U",312),("L",12),("D",746),("L",958),("U",216),("R",275),("D",278),("L",368),("U",663),("L",60),("D",543),("L",605),("D",991),("L",369),("D",599),("R",464),("D",387),("L",835),("D",876),("L",810),("U",377),("L",521),("U",113),("L",803),("U",680),("L",732),("D",449),("R",891),("D",558),("L",25),("U",249),("L",264),("U",643),("L",544),("U",504),("R",876),("U",403),("R",950),("U",19),("L",224),("D",287),("R",28),("U",914),("R",906),("U",970),("R",335),("U",295),("R",841),("D",810),("R",891),("D",596),("R",451),("D",79),("R",924),("U",823),("L",724),("U",968),("R",342),("D",349),("R",656),("U",373),("R",864),("U",374),("L",401),("D",102),("L",730),("D",886),("R",268),("D",188),("R",621),("U",258),("L",788),("U",408),("L",199),("D",422),("R",101),("U",368),("L",636),("U",543),("R",7),("U",722),("L",533),("U",242),("L",340),("D",195),("R",158),("D",291),("L",84),("U",936),("L",570),("D",937),("L",321),("U",947),("L",707),("U",32),("L",56),("U",650),("L",427),("U",490),("L",472),("U",258),("R",694),("U",87),("L",887),("U",575),("R",826),("D",398),("R",602),("U",794),("R",855),("U",225),("R",435),("U",591),("L",58),("U",281),("L",834),("D",400),("R",89),("D",201),("L",328),("U",278),("L",494),("D",70),("L",770),("D",182),("L",251),("D",44),("R",753),("U",431),("R",573),("D",71),("R",809),("U",983),("L",159),("U",26),("R",540),("U",516),("R",5),("D",23),("L",603),("U",65),("L",260),("D",187),("R",973),("U",877),("R",110),("U",49),("L",502),("D",68),("R",32),("U",153),("R",495),("D",315),("R",720),("D",439),("R",264),("D",603),("R",717),("U",586),("R",732),("D",111),("R",997),("U",578),("L",243),("U",256),("R",147),("D",425),("L",141),("U",758),("R",451),("U",779),("R",964),("D",219),("L",151),("D",789),("L",496),("D",484),("R",627),("D",431),("R",433),("D",761),("R",355),("U",975),("L",983),("U",364),("L",200),("U",578),("L",488),("U",668),("L",48),("D",774),("R",438),("D",456),("L",819),("D",927),("R",831),("D",598),("L",437),("U",979),("R",686),("U",930),("L",454),("D",553),("L",77),("D",955),("L",98),("U",201),("L",724),("U",211),("R",501),("U",492),("L",495),("U",732),("L",511)]));
			paths.push(("b",vec![("L",998),("U",949),("R",912),("D",186),("R",359),("D",694),("L",878),("U",542),("L",446),("D",118),("L",927),("U",175),("R",434),("U",473),("R",147),("D",54),("R",896),("U",890),("R",300),("D",537),("R",254),("D",322),("R",758),("D",690),("R",231),("U",269),("R",288),("U",968),("R",638),("U",192),("L",732),("D",355),("R",879),("U",451),("R",336),("D",872),("L",141),("D",842),("L",126),("U",584),("L",973),("D",940),("R",890),("D",75),("L",104),("U",340),("L",821),("D",590),("R",577),("U",859),("L",948),("D",199),("L",872),("D",751),("L",368),("U",506),("L",308),("U",827),("R",181),("U",94),("R",670),("U",901),("R",739),("D",48),("L",985),("D",801),("R",722),("D",597),("R",654),("D",606),("R",183),("U",646),("R",939),("U",677),("R",32),("U",936),("L",541),("D",934),("R",316),("U",354),("L",415),("D",930),("R",572),("U",571),("R",147),("D",609),("L",534),("D",406),("R",872),("D",527),("L",816),("D",960),("R",652),("D",429),("L",402),("D",858),("R",374),("D",930),("L",81),("U",106),("R",977),("U",251),("R",917),("U",966),("R",353),("U",732),("L",613),("U",280),("L",713),("D",937),("R",481),("U",52),("R",746),("U",203),("L",500),("D",557),("L",209),("U",249),("R",89),("D",58),("L",149),("U",872),("R",331),("D",460),("R",343),("D",423),("R",392),("D",160),("L",876),("U",981),("L",399),("D",642),("R",525),("U",515),("L",537),("U",113),("R",886),("D",516),("L",301),("D",680),("L",236),("U",399),("R",460),("D",869),("L",942),("D",280),("R",669),("U",476),("R",683),("D",97),("R",199),("D",444),("R",137),("D",489),("L",704),("D",120),("R",753),("D",100),("L",737),("U",375),("L",495),("D",325),("R",48),("D",269),("R",575),("U",895),("L",184),("D",10),("L",502),("D",610),("R",618),("D",744),("R",585),("U",861),("R",695),("D",775),("L",942),("U",64),("L",819),("U",161),("L",332),("U",513),("L",461),("D",366),("R",273),("D",493),("L",197),("D",97),("L",6),("U",63),("L",564),("U",59),("L",699),("U",30),("L",68),("U",861),("R",35),("U",564),("R",540),("U",371),("L",115),("D",595),("L",412),("D",781),("L",185),("D",41),("R",207),("D",264),("R",999),("D",799),("R",421),("D",117),("R",377),("D",571),("R",268),("D",947),("R",77),("D",2),("R",712),("D",600),("L",516),("U",389),("L",868),("D",762),("L",996),("U",205),("L",178),("D",339),("L",844),("D",629),("R",67),("D",732),("R",109),("D",858),("R",630),("U",470),("L",121),("D",542),("L",751),("U",353),("L",61),("U",770),("R",952),("U",703),("R",264),("D",537),("L",569),("U",55),("L",795),("U",389),("R",836),("U",166),("R",585),("U",275),("L",734),("U",966),("L",130),("D",357),("L",260),("U",719),("L",647),("D",606),("R",547),("U",575),("R",791),("U",686),("L",597),("D",486),("L",774),("U",386),("L",163),("U",912),("L",234),("D",238),("L",948),("U",279),("R",789),("U",300),("R",117),("D",28),("L",833),("U",835),("L",340),("U",693),("R",343),("D",573),("R",882),("D",241),("L",731),("U",812),("R",600),("D",663),("R",902),("U",402),("R",831),("D",802),("L",577),("U",920),("L",947),("D",538),("L",192)]));
		},
		_ => help!(args)
	}

	let mut panel = HashMap::new();
	for (id, path) in paths {
		let mut x: i32 = 0;
		let mut y: i32 = 0;
		for p in path {
			match p.0 {
				"R" => for _i in 0..p.1 {
					x += 1;
					trace_wire_v1!(panel, (x, y), id);
				},
				"L" => for _i in 0..p.1 {
					x -= 1;
					trace_wire_v1!(panel, (x, y), id);
				},
				"U" => for _i in 0..p.1 {
					y += 1;
					trace_wire_v1!(panel, (x, y), id);
				},
				"D" => for _i in 0..p.1 {
					y -= 1;
					trace_wire_v1!(panel, (x, y), id);
				},
				_ => unimplemented!()
			}
		}
	}

	let mut res = (0,0);
	let mut dist = std::i32::MAX;
	for (coords, val) in panel {
		if val.contains("a") && val.contains("b") {
			let d = coords.0.abs() + coords.1.abs();
			println!("{},{} =>\t{}, d = {}", coords.0, coords.1, val, d);
			if d < dist {
				res = coords;
				dist = d;
			}
		}
	}

	println!("Res {},{}\td = {}", res.0, res.1, dist);
}


macro_rules! trace_wire_v2 {
	($panel:expr, $coords:expr, $wire_id:expr, $step:expr) => {
		let val = $panel.entry($coords).or_insert( (String::new(),0) );
		if !val.0.contains($wire_id) {
			// println!("trace_wire_v2: x,y\t{}\t{}\twire={}\tstep={}", $coords.0, $coords.1, $wire_id, $step);
			val.0.push_str($wire_id);
			val.1 += $step;
			//$panel.insert($coords, val);
		}
	};
}
pub fn part2(args: Vec<String>) {
    println!("--- Day 3: Crossed Wires ---");
    println!("--- Part 2");

    if args.len() != 4 { help!(args); }

	let mut paths: Vec<(&str,Vec<(&str, usize)>)> = Vec::new();
	match args[3].as_ref() {
		// inputs test 1 => 6
		"test1" => {
			paths.push(("a",vec![("R",8),("U",5),("L",5),("D",3)]));
			paths.push(("b",vec![("U",7),("R",6),("D",4),("L",4)]));
		},
		// inputs test 2 => 159
		"test2" => {
			paths.push(("a",vec![("R",75),("D",30),("R",83),("U",83),("L",12),("D",49),("R",71),("U",7),("L",72)]));
			paths.push(("b",vec![("U",62),("R",66),("U",55),("R",34),("D",71),("R",55),("D",58),("R",83)]));
		},
		// inputs test 3 => 135
		"test3" => {
			paths.push(("a",vec![("R",98),("U",47),("R",26),("D",63),("R",33),("U",87),("L",62),("D",20),("R",33),("U",53),("R",51)]));
			paths.push(("b",vec![("U",98),("R",91),("D",20),("R",16),("D",67),("R",40),("U",7),("R",15),("U",6),("R",7)]));
		},
		// my inputs 
		"input" => {
			paths.push(("a",vec![("R",998),("U",367),("R",735),("U",926),("R",23),("U",457),("R",262),("D",473),("L",353),("U",242),("L",930),("U",895),("R",321),("U",683),("L",333),("U",623),("R",105),("D",527),("R",437),("D",473),("L",100),("D",251),("L",958),("U",384),("R",655),("U",543),("L",704),("D",759),("R",529),("D",176),("R",835),("U",797),("R",453),("D",650),("L",801),("U",437),("L",468),("D",841),("R",928),("D",747),("L",803),("U",677),("R",942),("D",851),("R",265),("D",684),("L",206),("U",763),("L",566),("U",774),("L",517),("U",337),("L",86),("D",585),("R",212),("U",656),("L",799),("D",953),("L",24),("U",388),("L",465),("U",656),("L",467),("U",649),("R",658),("U",519),("L",966),("D",290),("L",979),("D",819),("R",208),("D",907),("R",941),("D",458),("L",882),("U",408),("R",539),("D",939),("R",557),("D",771),("L",448),("U",460),("L",586),("U",148),("R",678),("U",360),("R",715),("U",312),("L",12),("D",746),("L",958),("U",216),("R",275),("D",278),("L",368),("U",663),("L",60),("D",543),("L",605),("D",991),("L",369),("D",599),("R",464),("D",387),("L",835),("D",876),("L",810),("U",377),("L",521),("U",113),("L",803),("U",680),("L",732),("D",449),("R",891),("D",558),("L",25),("U",249),("L",264),("U",643),("L",544),("U",504),("R",876),("U",403),("R",950),("U",19),("L",224),("D",287),("R",28),("U",914),("R",906),("U",970),("R",335),("U",295),("R",841),("D",810),("R",891),("D",596),("R",451),("D",79),("R",924),("U",823),("L",724),("U",968),("R",342),("D",349),("R",656),("U",373),("R",864),("U",374),("L",401),("D",102),("L",730),("D",886),("R",268),("D",188),("R",621),("U",258),("L",788),("U",408),("L",199),("D",422),("R",101),("U",368),("L",636),("U",543),("R",7),("U",722),("L",533),("U",242),("L",340),("D",195),("R",158),("D",291),("L",84),("U",936),("L",570),("D",937),("L",321),("U",947),("L",707),("U",32),("L",56),("U",650),("L",427),("U",490),("L",472),("U",258),("R",694),("U",87),("L",887),("U",575),("R",826),("D",398),("R",602),("U",794),("R",855),("U",225),("R",435),("U",591),("L",58),("U",281),("L",834),("D",400),("R",89),("D",201),("L",328),("U",278),("L",494),("D",70),("L",770),("D",182),("L",251),("D",44),("R",753),("U",431),("R",573),("D",71),("R",809),("U",983),("L",159),("U",26),("R",540),("U",516),("R",5),("D",23),("L",603),("U",65),("L",260),("D",187),("R",973),("U",877),("R",110),("U",49),("L",502),("D",68),("R",32),("U",153),("R",495),("D",315),("R",720),("D",439),("R",264),("D",603),("R",717),("U",586),("R",732),("D",111),("R",997),("U",578),("L",243),("U",256),("R",147),("D",425),("L",141),("U",758),("R",451),("U",779),("R",964),("D",219),("L",151),("D",789),("L",496),("D",484),("R",627),("D",431),("R",433),("D",761),("R",355),("U",975),("L",983),("U",364),("L",200),("U",578),("L",488),("U",668),("L",48),("D",774),("R",438),("D",456),("L",819),("D",927),("R",831),("D",598),("L",437),("U",979),("R",686),("U",930),("L",454),("D",553),("L",77),("D",955),("L",98),("U",201),("L",724),("U",211),("R",501),("U",492),("L",495),("U",732),("L",511)]));
			paths.push(("b",vec![("L",998),("U",949),("R",912),("D",186),("R",359),("D",694),("L",878),("U",542),("L",446),("D",118),("L",927),("U",175),("R",434),("U",473),("R",147),("D",54),("R",896),("U",890),("R",300),("D",537),("R",254),("D",322),("R",758),("D",690),("R",231),("U",269),("R",288),("U",968),("R",638),("U",192),("L",732),("D",355),("R",879),("U",451),("R",336),("D",872),("L",141),("D",842),("L",126),("U",584),("L",973),("D",940),("R",890),("D",75),("L",104),("U",340),("L",821),("D",590),("R",577),("U",859),("L",948),("D",199),("L",872),("D",751),("L",368),("U",506),("L",308),("U",827),("R",181),("U",94),("R",670),("U",901),("R",739),("D",48),("L",985),("D",801),("R",722),("D",597),("R",654),("D",606),("R",183),("U",646),("R",939),("U",677),("R",32),("U",936),("L",541),("D",934),("R",316),("U",354),("L",415),("D",930),("R",572),("U",571),("R",147),("D",609),("L",534),("D",406),("R",872),("D",527),("L",816),("D",960),("R",652),("D",429),("L",402),("D",858),("R",374),("D",930),("L",81),("U",106),("R",977),("U",251),("R",917),("U",966),("R",353),("U",732),("L",613),("U",280),("L",713),("D",937),("R",481),("U",52),("R",746),("U",203),("L",500),("D",557),("L",209),("U",249),("R",89),("D",58),("L",149),("U",872),("R",331),("D",460),("R",343),("D",423),("R",392),("D",160),("L",876),("U",981),("L",399),("D",642),("R",525),("U",515),("L",537),("U",113),("R",886),("D",516),("L",301),("D",680),("L",236),("U",399),("R",460),("D",869),("L",942),("D",280),("R",669),("U",476),("R",683),("D",97),("R",199),("D",444),("R",137),("D",489),("L",704),("D",120),("R",753),("D",100),("L",737),("U",375),("L",495),("D",325),("R",48),("D",269),("R",575),("U",895),("L",184),("D",10),("L",502),("D",610),("R",618),("D",744),("R",585),("U",861),("R",695),("D",775),("L",942),("U",64),("L",819),("U",161),("L",332),("U",513),("L",461),("D",366),("R",273),("D",493),("L",197),("D",97),("L",6),("U",63),("L",564),("U",59),("L",699),("U",30),("L",68),("U",861),("R",35),("U",564),("R",540),("U",371),("L",115),("D",595),("L",412),("D",781),("L",185),("D",41),("R",207),("D",264),("R",999),("D",799),("R",421),("D",117),("R",377),("D",571),("R",268),("D",947),("R",77),("D",2),("R",712),("D",600),("L",516),("U",389),("L",868),("D",762),("L",996),("U",205),("L",178),("D",339),("L",844),("D",629),("R",67),("D",732),("R",109),("D",858),("R",630),("U",470),("L",121),("D",542),("L",751),("U",353),("L",61),("U",770),("R",952),("U",703),("R",264),("D",537),("L",569),("U",55),("L",795),("U",389),("R",836),("U",166),("R",585),("U",275),("L",734),("U",966),("L",130),("D",357),("L",260),("U",719),("L",647),("D",606),("R",547),("U",575),("R",791),("U",686),("L",597),("D",486),("L",774),("U",386),("L",163),("U",912),("L",234),("D",238),("L",948),("U",279),("R",789),("U",300),("R",117),("D",28),("L",833),("U",835),("L",340),("U",693),("R",343),("D",573),("R",882),("D",241),("L",731),("U",812),("R",600),("D",663),("R",902),("U",402),("R",831),("D",802),("L",577),("U",920),("L",947),("D",538),("L",192)]));
		},
		_ => help!(args)
	}

	let mut panel = HashMap::new();
	for (id, path) in paths {
		let mut x: i32 = 0;
		let mut y: i32 = 0;
		let mut i = 0;
		for p in path {
			match p.0 {
				"R" => for _i in 0..p.1 {
					i+=1;
					x += 1;
					trace_wire_v2!(panel, (x, y), id, i);
				},
				"L" => for _i in 0..p.1 {
					i+=1;
					x -= 1;
					trace_wire_v2!(panel, (x, y), id, i);
				},
				"U" => for _i in 0..p.1 {
					i+=1;
					y += 1;
					trace_wire_v2!(panel, (x, y), id, i);
				},
				"D" => for _i in 0..p.1 {
					i+=1;
					y -= 1;
					trace_wire_v2!(panel, (x, y), id, i);
				},
				_ => unimplemented!()
			}
		}
	}

	let mut res = (0,0);
	let mut step = std::i32::MAX;
	let mut dist = std::i32::MAX;
	for (coords, val) in panel {
		if val.0.contains("a") && val.0.contains("b") {
			let d = coords.0.abs() + coords.1.abs();
			println!("{},{} =>\t{},{}, d = {}", coords.0, coords.1, val.0, val.1, d);
			if val.1 < step {
				res = coords;
				step = val.1;
				dist = d;
			}
		}
	}

	println!("Res {},{}\td = {}\tstep = {}", res.0, res.1, dist, step);
}
