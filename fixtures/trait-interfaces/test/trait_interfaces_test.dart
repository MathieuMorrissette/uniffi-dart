import 'package:test/test.dart';
import '../trait_interfaces.dart';

void main() {
  group('FriendlyGreeter', () {
    test('toTrait produces a Greeter handle', () {
      final friendly = FriendlyGreeter('Hello');
      final Greeter greeter = friendly.toTrait();
      expect(greeter.greet('World'), equals('Hello World'));
      greeter.dispose();
    });

    test('equality and hashing honour Rust traits', () {
      final a = FriendlyGreeter('Hi');
      final b = FriendlyGreeter('Hi');
      final c = FriendlyGreeter('Hey');

      expect(a, equals(b));
      expect(a.hashCode, equals(b.hashCode));
      expect(a, isNot(equals(c)));

      final map = <FriendlyGreeter, String>{a: 'value'};
      expect(map.containsKey(b), isTrue);
    });
  });

  group('Registry', () {
    test('returns Greeter trait objects', () {
      final registry = Registry();

      final Greeter friendly = registry.makeFriendly('Hi');
      final Greeter proc = registry.makeProc('hola');

      expect(friendly.greet('there'), equals('Hi there'));
      expect(proc.greet('mundo'), equals('HOLA MUNDO'));

      friendly.dispose();
      proc.dispose();
    });
  });
}
