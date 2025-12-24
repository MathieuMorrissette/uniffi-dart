import 'dart:typed_data';
import 'package:test/test.dart';
import '../proc_macro_pure_uniffi.dart';

void main() {
  group('Record tests', () {
    test('create person', () {
      final person = createPerson('Alice', 30);
      expect(person.name, equals('Alice'));
      expect(person.age, equals(30));
    });

    test('greet person', () {
      final person = createPerson('Bob', 25);
      final greeting = greet(person);
      expect(greeting, equals('Hello, Bob! You are 25 years old.'));
    });
  });

  group('Enum tests', () {
    test('status to string', () {
      expect(statusToString(UserStatus.active), equals('Active'));
      expect(statusToString(UserStatus.inactive), equals('Inactive'));
      expect(statusToString(UserStatus.pending), equals('Pending'));
    });
  });

  group('Object tests', () {
    test('counter creation and get', () {
      final counter = Counter(0);
      expect(counter.getValue(), equals(0));
    });

    test('counter increment', () {
      final counter = Counter(5);
      expect(counter.getValue(), equals(5));
      counter.increment();
      expect(counter.getValue(), equals(6));
      counter.increment();
      expect(counter.getValue(), equals(7));
    });

    test('multiple counters', () {
      final counter1 = Counter(10);
      final counter2 = Counter(20);

      counter1.increment();
      expect(counter1.getValue(), equals(11));
      expect(counter2.getValue(), equals(20));

      counter2.increment();
      counter2.increment();
      expect(counter1.getValue(), equals(11));
      expect(counter2.getValue(), equals(22));
    });
  });

  group('Default parameter tests', () {
    test('hash with all defaults', () {
      final data = Uint8List.fromList([1, 2, 3, 4, 5]);
      final result = hashData(data);
      expect(result, isNotNull);
      expect(result.length, equals(32)); // default length
    });

    test('hash with custom iterations', () {
      final data = Uint8List.fromList([1, 2, 3, 4, 5]);
      final result = hashData(data, iterations: 5000);
      expect(result, isNotNull);
      expect(result.length, equals(32)); // default length
    });

    test('hash with custom length', () {
      final data = Uint8List.fromList([1, 2, 3, 4, 5]);
      final result = hashData(data, length: 64);
      expect(result, isNotNull);
      expect(result.length, equals(64)); // custom length
    });

    test('hash with all parameters specified', () {
      final data = Uint8List.fromList([1, 2, 3, 4, 5]);
      final result = hashData(data, iterations: 5000, length: 16);
      expect(result, isNotNull);
      expect(result.length, equals(16)); // custom length
    });
  });
}
