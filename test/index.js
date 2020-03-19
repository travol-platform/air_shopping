/// NB: The tryorama config patterns are still not quite stabilized.
/// See the tryorama README [https://github.com/holochain/tryorama]
/// for a potentially more accurate example
const path = require("path");
const {
  Orchestrator,
  Config,
  combine,
  localOnly,
  tapeExecutor
} = require("@holochain/tryorama");

process.on("unhandledRejection", error => {
  // Will print "unhandledRejection err is not defined"
  console.error("got unhandledRejection:", error);
});

const dnaPath = path.join(__dirname, "../dist/air_shopping.dna.json");

const orchestrator = new Orchestrator({
  middleware: combine(
    // use the tape harness to run the tests, injects the tape API into each scenario
    // as the second argument
    tapeExecutor(require("tape")),

    // specify that all "players" in the test are on the local machine, rather than
    // on remote machines
    localOnly
  )
});

const dna = Config.dna(dnaPath, "h_air");
const config = Config.gen(
  {
    h_air: dna
  },
  {
    network: {
      type: "sim2h",
      sim2h_url: "ws://localhost:9000"
    }
  }
);


orchestrator.registerScenario("create fligth segment", async (s, t) => {
  const { alice } = await s.players({ alice: config }, true);
  const expect = {
    secure_flight: true,
    segment_key: "aaa",
    departure: {
      airport_code: "String",
      timestamp: "String",
      airport_name: "String",
      terminal_name: "String"
    },
    arrival: {
      airport_code: "hola",
      timestamp: "hola",
      change_of_day: "hola",
      airport_name: "hola",
      terminal_name: "hola"
    },
    marketing_carrier: {
      airline_id: "String",
      name: "String",
      flight_number: "String"
    },
    operation_carrier: { airline_id: "String", name: "String" },
    equipement: {
      aircraft_code: "2",
      name: "String"
    },
    class_of_service: {
      ref: "holass ", code: '32', seats_left: "9",
      markting_name: { cabin_designator: "32", name: "String" }
    },
    flight_detail: {
      flight_segment_type: "String",
      flight_duration: "String",
      stops: "32",
      stop_location: [{
        airport_code: "String",
        arrival_timestamp: "String",
        departure_timestamp: "String",
      }]
    }
  };
  const result = await alice.call(
    "h_air",
    "air_shopping",
    "create_flight_segment",
    { flight_segment: expect }
  );
  await s.consistency()
  const result2 = await alice.call(
    "h_air",
    "air_shopping",
    "get_entry",
    { type: "flight_segment", key: expect.segment_key }
  );
  t.ok(result.Ok);
  t.deepEqual(result2.Ok, expect);
});
orchestrator.registerScenario("create fligth segment", async (s, t) => {
  const { alice } = await s.players({ alice: config }, true);
  const expect = {
    refs: "true",
    list_key: "bbdd",
    fare_code: "String",
    fare_basis_code: "String",

  };
  const result = await alice.call(
    "h_air",
    "air_shopping",
    "create_fare",
    { fare: expect }
  );
  await s.consistency()
  const result2 = await alice.call(
    "h_air",
    "air_shopping",
    "get_entry",
    { type: "fare", key: expect.list_key }
  );
  t.ok(result.Ok);
  t.deepEqual(result2.Ok, expect);
});
orchestrator.registerScenario("create fligth segment", async (s, t) => {
  const expect_flight_segment = {
    secure_flight: true,
    segment_key: "aaa",
    departure: {
      airport_code: "String",
      timestamp: "String",
      airport_name: "String",
      terminal_name: "String"
    },
    arrival: {
      airport_code: "hola",
      timestamp: "hola",
      change_of_day: "hola",
      airport_name: "hola",
      terminal_name: "hola"
    },
    marketing_carrier: {
      airline_id: "String",
      name: "String",
      flight_number: "String"
    },
    operation_carrier: { airline_id: "String", name: "String" },
    equipement: {
      aircraft_code: "2",
      name: "String"
    },
    class_of_service: {
      ref: "holass ", code: '32', seats_left: "9",
      markting_name: { cabin_designator: "32", name: "String" }
    },
    flight_detail: {
      flight_segment_type: "String",
      flight_duration: "String",
      stops: "32",
      stop_location: [{
        airport_code: "String",
        arrival_timestamp: "String",
        departure_timestamp: "String",
      }]
    }
  };
  const { alice } = await s.players({ alice: config }, true);
  const result = await alice.call(
    "h_air",
    "air_shopping",
    "create_flight_segment",
    { flight_segment: expect_flight_segment }
  );
  await s.consistency()
  const expect_fare = {
    refs: "true",
    list_key: "bbdd",
    fare_code: "String",
    fare_basis_code: "String",

  };
  const result2 = await alice.call(
    "h_air",
    "air_shopping",
    "create_fare",
    { fare: expect_fare }
  );
  await s.consistency()
  const expect_price_class = {
    price_class_id: "true",
    name: "bbdd",
    descriptions: ["String"],
    class_of_service: {
      ref: `${expect_flight_segment.segment_key} ${expect_fare.list_key}`,
      code: "String",
      seats_left: "Option<String>",
      markting_name: {
        abin_designator: "Option<String>",
        name: "Option<String>",
      },
    },

  };
  const result3 = await alice.call(
    "h_air",
    "air_shopping",
    "create_price_class",
    { price_class: expect_price_class }
  );
  t.ok(result.Ok);
  t.ok(result2.Ok);
  t.ok(result3.Ok);

});
orchestrator.run();
