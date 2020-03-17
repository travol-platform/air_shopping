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

orchestrator.registerScenario("Test hello holo", async (s, t) => {
  const { alice, bob } = await s.players({ alice: config, bob: config }, true);
  const expect = {
    secure_flight: true,
    segment_key: "holssd",
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
    { type: "flight_segment", key: "holssd" }
  );
  t.ok(result.Ok);
  delete result2.Ok.anchor_address;
  t.deepEqual(result2.Ok, expect);
});
orchestrator.run();
