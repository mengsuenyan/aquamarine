/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use aqua_test_utils::call_vm;
use aqua_test_utils::create_aqua_vm;
use aqua_test_utils::unit_call_service;
use aqua_test_utils::CallServiceClosure;
use aqua_test_utils::IValue;
use aqua_test_utils::NEVec;

use pretty_assertions::assert_eq;
use serde_json::json;

type JValue = serde_json::Value;

#[test]
fn join_chat() {
    use std::collections::HashSet;

    let members_call_service1: CallServiceClosure = Box::new(|_, _| -> Option<IValue> {
        Some(IValue::Record(
            NEVec::new(vec![
                IValue::S32(0),
                IValue::String(String::from(r#"[["A", "Relay1"], ["B", "Relay2"]]"#)),
            ])
            .unwrap(),
        ))
    });

    let mut relay_1 = create_aqua_vm(unit_call_service(), "Relay1");
    let mut relay_2 = create_aqua_vm(unit_call_service(), "Relay2");
    let mut remote = create_aqua_vm(members_call_service1, "Remote");
    let mut client_1 = create_aqua_vm(unit_call_service(), "A");
    let mut client_2 = create_aqua_vm(unit_call_service(), "B");

    let script = String::from(
        r#"
            (seq
                (call "Relay1" ("identity" "") [] void1[])
                (seq
                    (call "Remote" ("552196ea-b9b2-4761-98d4-8e7dba77fac4" "add") [] void2[])
                    (seq
                        (call "Remote" ("920e3ba3-cbdf-4ae3-8972-0fa2f31fffd9" "get_users") [] members)
                        (fold members m
                            (par 
                                (seq
                                    (call m.$.[1] ("identity" "") [] void[])
                                    (call m.$.[0] ("fgemb3" "add") [] void3[])
                                )
                                (next m)
                            )
                        )
                    )
                )
            )
        "#,
    );

    let client_1_res = call_vm!(client_1, "asd", script.clone(), "[]", "[]");

    let client_1_res_json: JValue =
        serde_json::from_slice(&client_1_res.data).expect("stepper should return valid json");

    let client_1_expected_json = json!([
        { "call": {"request_sent_by": "A" } },
    ]);

    assert_eq!(client_1_res_json, client_1_expected_json);
    assert_eq!(client_1_res.next_peer_pks, vec![String::from("Relay1")]);

    let relay_1_res = call_vm!(relay_1, "asd", script.clone(), client_1_res.data, "[]");

    let relay_1_res_json: JValue = serde_json::from_slice(&relay_1_res.data).expect("stepper should return valid json");

    let relay_1_expected_json = json!( [
        { "call": { "executed" : "test" } },
        { "call": { "request_sent_by": "Relay1" } },
    ]);

    assert_eq!(relay_1_res_json, relay_1_expected_json);
    assert_eq!(relay_1_res.next_peer_pks, vec![String::from("Remote")]);

    let remote_res = call_vm!(remote, "asd", script.clone(), relay_1_res.data, "[]");

    let remote_res_json: JValue = serde_json::from_slice(&remote_res.data).expect("stepper should return valid json");

    let remote_expected_json = json!( [
        { "call": { "executed" : "test" } },
        { "call": { "executed" : [["A", "Relay1"], ["B", "Relay2"]]} },
        { "call": { "executed" : [["A", "Relay1"], ["B", "Relay2"]]} },
        { "par": [1, 2] },
        { "call": { "request_sent_by" : "Remote" } },
        { "par": [1, 0] },
        { "call": { "request_sent_by" : "Remote" } },
    ]);

    let remote_res_next_peer_pks: HashSet<_> = remote_res.next_peer_pks.iter().map(|s| s.as_str()).collect();
    let next_peer_pks_right = maplit::hashset! {
        "Relay1",
        "Relay2",
    };

    assert_eq!(remote_res_json, remote_expected_json);
    assert_eq!(remote_res_next_peer_pks, next_peer_pks_right);

    let relay_1_res = call_vm!(relay_1, "asd", script.clone(), remote_res.data.clone(), "[]");

    let relay_1_res_json: JValue = serde_json::from_slice(&relay_1_res.data).expect("stepper should return valid json");

    let relay_1_expected_json = json!( [
        { "call": { "executed" : "test" } },
        { "call": { "executed" : [["A", "Relay1"], ["B", "Relay2"]]} },
        { "call": { "executed" : [["A", "Relay1"], ["B", "Relay2"]]} },
        { "par": [2, 2] },
        { "call": { "executed" : "test" } },
        { "call": { "request_sent_by" : "Relay1" } },
        { "par": [1, 0] },
        { "call": { "request_sent_by" : "Remote" } },
    ]);

    assert_eq!(relay_1_res_json, relay_1_expected_json);
    assert_eq!(relay_1_res.next_peer_pks, vec![String::from("A")]);

    let client_1_res = call_vm!(client_1, "asd", script.clone(), relay_1_res.data, "[]");

    let client_1_res_json: JValue =
        serde_json::from_slice(&client_1_res.data).expect("stepper should return valid json");

    let client_1_expected_json = json!( [
        { "call": { "executed" : "test" } },
        { "call": { "executed" : [["A", "Relay1"], ["B", "Relay2"]]} },
        { "call": { "executed" : [["A", "Relay1"], ["B", "Relay2"]]} },
        { "par": [2, 2] },
        { "call": { "executed" : "test" } },
        { "call": { "executed" : "test" } },
        { "par": [1, 0] },
        { "call": { "request_sent_by" : "Remote" } },
    ]);

    assert_eq!(client_1_res_json, client_1_expected_json);
    assert_eq!(client_1_res.next_peer_pks, Vec::<String>::new());

    let relay_2_res = call_vm!(relay_2, "asd", script.clone(), remote_res.data, "[]");

    let relay_2_res_json: JValue = serde_json::from_slice(&relay_2_res.data).expect("stepper should return valid json");

    let relay_2_expected_json = json!( [
        { "call": { "executed" : "test" } },
        { "call": { "executed" : [["A", "Relay1"], ["B", "Relay2"]]} },
        { "call": { "executed" : [["A", "Relay1"], ["B", "Relay2"]]} },
        { "par": [1, 3] },
        { "call": { "request_sent_by" : "Remote" } },
        { "par": [2, 0] },
        { "call": { "executed" : "test" } },
        { "call": { "request_sent_by" : "Relay2" } },
    ]);

    assert_eq!(relay_2_res_json, relay_2_expected_json);
    assert_eq!(relay_2_res.next_peer_pks, vec![String::from("B")]);

    let client_2_res = call_vm!(client_2, "asd", script, relay_2_res.data, "[]");

    let client_2_res_json: JValue =
        serde_json::from_slice(&client_2_res.data).expect("stepper should return valid json");

    let client_2_expected_json = json!( [
        { "call": { "executed" : "test" } },
        { "call": { "executed" : [["A", "Relay1"], ["B", "Relay2"]]} },
        { "call": { "executed" : [["A", "Relay1"], ["B", "Relay2"]]} },
        { "par": [1, 3] },
        { "call": { "request_sent_by" : "Remote" } },
        { "par": [2, 0] },
        { "call": { "executed" : "test" } },
        { "call": { "executed" : "test" } },
    ]);

    assert_eq!(client_2_res_json, client_2_expected_json);
    assert_eq!(client_2_res.next_peer_pks, Vec::<String>::new());
}

#[test]
fn join() {
    let members_call_service1: CallServiceClosure = Box::new(|_, _| -> Option<IValue> {
        Some(IValue::Record(
            NEVec::new(vec![IValue::S32(0), IValue::String(String::from(r#"[["A"], ["B"]]"#))]).unwrap(),
        ))
    });

    let mut relay_1 = create_aqua_vm(unit_call_service(), "Relay1");
    let mut remote = create_aqua_vm(members_call_service1, "Remote");
    let mut client_1 = create_aqua_vm(unit_call_service(), "A");

    let script = String::from(
        r#"
            (seq
                (call "Relay1" ("identity" "") [] void1[])
                (seq
                    (call "Remote" ("920e3ba3-cbdf-4ae3-8972-0fa2f31fffd9" "get_users") [] members)
                    (fold members m
                        (par 
                            (seq
                                (call "Relay1" ("identity" "") [] void[])
                                (call "A" ("fgemb3" "add") [m] void3[])
                            )
                            (next m)
                        )
                    )
                )
            )
        "#,
    );

    let client_1_res = call_vm!(client_1, "asd", script.clone(), "[]", "[]");
    let relay_1_res = call_vm!(relay_1, "asd", script.clone(), client_1_res.data, "[]");
    let remote_res = call_vm!(remote, "asd", script.clone(), relay_1_res.data, "[]");
    let relay_1_res = call_vm!(relay_1, "asd", script.clone(), remote_res.data, "[]");
    let client_1_res = call_vm!(client_1, "asd", script, relay_1_res.data, "[]");

    let client_1_res_json: JValue =
        serde_json::from_slice(&client_1_res.data).expect("stepper should return valid json");

    let client_1_expected_json = json!( [
        { "call": { "executed" : "test" } },
        { "call": { "executed" : [["A"], ["B"]]} },
        { "par": [2, 3] },
        { "call": { "executed" : "test" } },
        { "call": { "executed" : "test" } },
        { "par": [2, 0] },
        { "call": { "executed" : "test" } },
        { "call": { "executed" : "test" } },
    ]);

    assert_eq!(client_1_res_json, client_1_expected_json);
    assert_eq!(client_1_res.next_peer_pks, Vec::<String>::new());
}

#[test]
fn init_peer_id() {
    let members_call_service1: CallServiceClosure = Box::new(|_, _| -> Option<IValue> {
        Some(IValue::Record(
            NEVec::new(vec![IValue::S32(0), IValue::String(String::from(r#"[["A"], ["B"]]"#))]).unwrap(),
        ))
    });

    let initiator_peer_id = String::from("initiator");

    let mut relay_1 = create_aqua_vm(unit_call_service(), "Relay1");
    let mut remote = create_aqua_vm(members_call_service1, "Remote");
    let mut client_1 = create_aqua_vm(unit_call_service(), "A");
    let mut initiator = create_aqua_vm(unit_call_service(), initiator_peer_id.clone());

    let script = String::from(
        r#"(seq
                (seq
                    (call "Relay1" ("identity" "") [])
                    (seq
                        (call "Remote" ("920e3ba3-cbdf-4ae3-8972-0fa2f31fffd9" "get_users") [] members)
                        (fold members m
                            (par
                                (seq
                                    (call "Relay1" ("identity" "") [])
                                    (call "A" ("fgemb3" "add") [m])
                                )
                                (next m)
                            )
                        )
                    )
                )
                (call %init_peer_id% ("identity" "") [])
            )
        "#,
    );

    let initiator_1_res = call_vm!(initiator, initiator_peer_id.clone(), script.clone(), "", "");
    let client_1_res = call_vm!(
        client_1,
        initiator_peer_id.clone(),
        script.clone(),
        initiator_1_res.data,
        ""
    );
    let relay_1_res = call_vm!(
        relay_1,
        initiator_peer_id.clone(),
        script.clone(),
        client_1_res.data,
        ""
    );
    let remote_res = call_vm!(remote, initiator_peer_id.clone(), script.clone(), relay_1_res.data, "");
    let relay_1_res = call_vm!(relay_1, initiator_peer_id.clone(), script.clone(), remote_res.data, "");
    let client_1_res = call_vm!(
        client_1,
        initiator_peer_id.clone(),
        script.clone(),
        relay_1_res.data,
        ""
    );

    let client_1_res_json: JValue =
        serde_json::from_slice(&client_1_res.data).expect("stepper should return valid json");

    let client_1_expected_json = json!( [
        { "call": { "executed" : "test" } },
        { "call": { "executed" : [["A"], ["B"]]} },
        { "par": [2, 3] },
        { "call": { "executed" : "test" } },
        { "call": { "executed" : "test" } },
        { "par": [2, 0] },
        { "call": { "executed" : "test" } },
        { "call": { "executed" : "test" } },
        { "call": { "request_sent_by" : "A" } },
    ]);

    assert_eq!(client_1_res_json, client_1_expected_json);
    assert_eq!(client_1_res.next_peer_pks, vec![initiator_peer_id.clone()]);

    let initiator_1_res = call_vm!(initiator, initiator_peer_id, script, client_1_res.data, "");

    let initiator_1_res_json: JValue =
        serde_json::from_slice(&initiator_1_res.data).expect("stepper should return valid json");

    let initiator_1_expected_json = json!( [
        { "call": { "executed" : "test" } },
        { "call": { "executed" : [["A"], ["B"]]} },
        { "par": [2, 3] },
        { "call": { "executed" : "test" } },
        { "call": { "executed" : "test" } },
        { "par": [2, 0] },
        { "call": { "executed" : "test" } },
        { "call": { "executed" : "test" } },
        { "call": { "executed" : "test" } },
    ]);

    assert_eq!(initiator_1_res_json, initiator_1_expected_json);
    assert_eq!(initiator_1_res.next_peer_pks, Vec::<String>::new());
}
